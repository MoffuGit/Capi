import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const sendFriendRequest = mutation({
  args: {
    receiverId: v.id("users"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const sender = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .unique();

    if (!sender) {
      throw new Error("Sender not found");
    }

    if (sender._id === args.receiverId) {
      throw new Error("Cannot send friend request to yourself");
    }

    const existingFriendship = await ctx.db
      .query("friends")
      .withIndex("by_sender_receiver", (q) =>
        q.eq("sender", sender._id).eq("receiver", args.receiverId),
      )
      .first();

    if (existingFriendship) {
      if (existingFriendship.status === "pending") {
        throw new Error("Friend request already sent");
      }
      if (existingFriendship.status === "accepted") {
        throw new Error("Already friends");
      }
    }

    // Check for a reverse request (receiver sent request to sender)
    const reverseFriendship = await ctx.db
      .query("friends")
      .withIndex("by_sender_receiver", (q) =>
        q.eq("sender", args.receiverId).eq("receiver", sender._id),
      )
      .first();

    if (reverseFriendship) {
      if (reverseFriendship.status === "pending") {
        // Automatically accept the request if a reverse pending request exists
        await ctx.db.patch(reverseFriendship._id, { status: "accepted" });
        return {
          message: "Friend request accepted automatically",
          friendshipId: reverseFriendship._id,
        };
      }
      if (reverseFriendship.status === "accepted") {
        throw new Error("Already friends");
      }
    }

    return await ctx.db.insert("friends", {
      sender: sender._id,
      receiver: args.receiverId,
      status: "pending",
    });
  },
});

export const acceptFriendRequest = mutation({
  args: {
    requestId: v.id("friends"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .unique();

    if (!currentUser) {
      throw new Error("Current user not found");
    }

    const request = await ctx.db.get(args.requestId);

    if (
      !request ||
      request.receiver !== currentUser._id ||
      request.status !== "pending"
    ) {
      throw new Error("Friend request not found or not pending for this user");
    }

    await ctx.db.patch(args.requestId, { status: "accepted" });

    // Create a reverse entry for easier querying for both users
    const existingReverseFriendship = await ctx.db
      .query("friends")
      .withIndex("by_sender_receiver", (q) =>
        q.eq("sender", request.receiver).eq("receiver", request.sender),
      )
      .first();

    if (!existingReverseFriendship) {
      await ctx.db.insert("friends", {
        sender: request.receiver,
        receiver: request.sender,
        status: "accepted",
      });
    } else if (existingReverseFriendship.status === "pending") {
      // If a reverse request was pending, accept it. This should be handled by sendFriendRequest, but as a safeguard.
      await ctx.db.patch(existingReverseFriendship._id, { status: "accepted" });
    }

    return { success: true };
  },
});

// Decline or remove a friend request/friendship
export const declineFriendRequest = mutation({
  args: {
    friendshipId: v.id("friends"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .unique();

    if (!currentUser) {
      throw new Error("Current user not found");
    }

    const friendship = await ctx.db.get(args.friendshipId);

    if (
      !friendship ||
      (friendship.sender !== currentUser._id &&
        friendship.receiver !== currentUser._id)
    ) {
      throw new Error("Friendship not found or unauthorized");
    }

    // Delete the friendship entry
    await ctx.db.delete(args.friendshipId);

    // Also delete the reverse entry if it exists and is an accepted friendship
    if (friendship.status === "accepted") {
      const reverseFriendship = await ctx.db
        .query("friends")
        .withIndex("by_sender_receiver", (q) =>
          q.eq("sender", friendship.receiver).eq("receiver", friendship.sender),
        )
        .first();

      if (reverseFriendship) {
        await ctx.db.delete(reverseFriendship._id);
      }
    }

    return { success: true };
  },
});

// Get all accepted friends for the current user
export const getFriends = query({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .unique();

    if (!currentUser) {
      return [];
    }

    const friendships = await ctx.db
      .query("friends")
      .withIndex("by_sender", (q) => q.eq("sender", currentUser._id))
      .filter((q) => q.eq(q.field("status"), "accepted"))
      .collect();

    const friendUserIds = friendships.map((f) => f.receiver);

    const friendsWithDetails = await Promise.all(
      friendUserIds.map(async (userId) => {
        const user = await ctx.db.get(userId);
        return user
          ? { _id: user._id, name: user.name, imageUrl: user.image_url }
          : null;
      }),
    );

    return friendsWithDetails.filter(Boolean);
  },
});

export const getPendingFriendRequests = query({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!currentUser) {
      return [];
    }

    const requests = await ctx.db
      .query("friends")
      .withIndex("by_receiver", (q) => q.eq("receiver", currentUser._id))
      .filter((q) => q.eq(q.field("status"), "pending"))
      .collect();

    const requestsWithSenderDetails = await Promise.all(
      requests.map(async (request) => {
        const sender = await ctx.db.get(request.sender);
        return sender
          ? {
              _id: request._id,
              senderId: sender._id,
              senderName: sender.name,
              senderImageUrl: sender.image_url,
              status: request.status,
            }
          : null;
      }),
    );

    return requestsWithSenderDetails.filter(Boolean);
  },
});

// Get outgoing pending friend requests from the current user
export const getSentFriendRequests = query({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!currentUser) {
      return [];
    }

    const requests = await ctx.db
      .query("friends")
      .withIndex("by_sender", (q) => q.eq("sender", currentUser._id))
      .filter((q) => q.eq(q.field("status"), "pending"))
      .collect();

    const requestsWithReceiverDetails = await Promise.all(
      requests.map(async (request) => {
        const receiver = await ctx.db.get(request.receiver);
        return receiver
          ? {
              _id: request._id,
              receiverId: receiver._id,
              receiverName: receiver.name,
              receiverImageUrl: receiver.image_url,
              status: request.status,
            }
          : null;
      }),
    );

    return requestsWithReceiverDetails.filter(Boolean);
  },
});

export const getFriendshipStatus = query({
  args: {
    userId: v.id("users"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!currentUser) {
      return "user_not_found";
    }

    if (currentUser._id === args.userId) {
      return "self";
    }

    // Check if they are friends
    const friendship = await ctx.db
      .query("friends")
      .withIndex("by_sender_receiver", (q) =>
        q.eq("sender", currentUser._id).eq("receiver", args.userId),
      )
      .first();

    if (friendship && friendship.status === "accepted") {
      return "friends";
    }

    // Check if a request has been sent by current user
    if (friendship && friendship.status === "pending") {
      return "request_sent";
    }

    // Check if a request has been received by current user
    const reverseFriendship = await ctx.db
      .query("friends")
      .withIndex("by_sender_receiver", (q) =>
        q.eq("sender", args.userId).eq("receiver", currentUser._id),
      )
      .first();

    if (reverseFriendship && reverseFriendship.status === "pending") {
      return "request_received";
    }

    return "not_friends";
  },
});
