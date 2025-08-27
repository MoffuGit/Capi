import { v } from "convex/values";
import { mutation, query } from "./_generated/server";
import { type Id } from "./_generated/dataModel";

// Create or get a private conversation
export const createOrGetConversation = mutation({
  args: {
    member2Id: v.id("users"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const member1 = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!member1) {
      throw new Error("Member 1 not found");
    }

    if (member1._id === args.member2Id) {
      throw new Error("Cannot create conversation with yourself");
    }

    // Ensure memberOne is always the "smaller" ID for consistent querying
    const [memberOneId, memberTwoId] =
      member1._id < args.member2Id
        ? [member1._id, args.member2Id]
        : [args.member2Id, member1._id];

    // Check if a conversation already exists between these two members
    const existingConversation = await ctx.db
      .query("conversations")
      .withIndex("by_memberOne_memberTwo", (q) =>
        q.eq("memberOne", memberOneId).eq("memberTwo", memberTwoId),
      )
      .first();

    if (existingConversation) {
      return existingConversation._id;
    }

    // If no conversation exists, create a new one
    const conversationId = await ctx.db.insert("conversations", {
      memberOne: memberOneId,
      memberTwo: memberTwoId,
    });

    return conversationId;
  },
});

// Send a private message
export const sendPrivateMessage = mutation({
  args: {
    conversationId: v.id("conversations"),
    content: v.string(),
    referenceId: v.optional(v.id("privateMessages")),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const sender = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!sender) {
      throw new Error("Sender not found");
    }

    const conversation = await ctx.db.get(args.conversationId);

    if (
      !conversation ||
      (conversation.memberOne !== sender._id &&
        conversation.memberTwo !== sender._id)
    ) {
      throw new Error("Conversation not found or user not a member");
    }

    return await ctx.db.insert("privateMessages", {
      conversation: args.conversationId,
      sender: sender._id,
      content: args.content,
      reference: args.referenceId,
    });
  },
});

// Get private messages for a conversation
export const getPrivateMessages = query({
  args: {
    conversationId: v.id("conversations"),
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

    const conversation = await ctx.db.get(args.conversationId);

    if (
      !conversation ||
      (conversation.memberOne !== currentUser._id &&
        conversation.memberTwo !== currentUser._id)
    ) {
      return [];
    }

    const messages = await ctx.db
      .query("privateMessages")
      .withIndex("by_conversation", (q) =>
        q.eq("conversation", args.conversationId),
      )
      .collect();

    const messagesWithSender = await Promise.all(
      messages.map(async (message) => {
        const sender = await ctx.db.get(message.sender);
        return sender
          ? {
              ...message,
              senderName: sender.name,
              senderImageUrl: sender.image_url,
            }
          : null;
      }),
    );

    return messagesWithSender.filter(Boolean);
  },
});

// Mark messages in a conversation as read up to a specific message
export const markPrivateMessagesRead = mutation({
  args: {
    conversationId: v.id("conversations"),
    lastReadMessageId: v.id("privateMessages"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!currentUser) {
      throw new Error("Current user not found");
    }

    const conversation = await ctx.db.get(args.conversationId);

    if (
      !conversation ||
      (conversation.memberOne !== currentUser._id &&
        conversation.memberTwo !== currentUser._id)
    ) {
      throw new Error("Conversation not found or user not a member");
    }

    const existingReadEntry = await ctx.db
      .query("privateMessageReads")
      .withIndex("by_member_and_conversation", (q) =>
        q.eq("member", currentUser._id).eq("conversation", args.conversationId),
      )
      .first();

    if (existingReadEntry) {
      await ctx.db.patch(existingReadEntry._id, {
        lastReadMessage: args.lastReadMessageId,
      });
    } else {
      await ctx.db.insert("privateMessageReads", {
        member: currentUser._id,
        conversation: args.conversationId,
        lastReadMessage: args.lastReadMessageId,
      });
    }

    return { success: true };
  },
});

// Get the last read message for a user in a conversation
export const getLastReadMessage = query({
  args: {
    conversationId: v.id("conversations"),
    auth: v.int64(),
  },
  handler: async (ctx, args) => {
    const currentUser = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", args.auth))
      .first();

    if (!currentUser) {
      return null;
    }

    const readEntry = await ctx.db
      .query("privateMessageReads")
      .withIndex("by_member_and_conversation", (q) =>
        q.eq("member", currentUser._id).eq("conversation", args.conversationId),
      )
      .first();

    return readEntry ? readEntry.lastReadMessage : null;
  },
});

// Get all private conversations for the current user
export const getMyConversations = query({
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

    // Define a local async helper function to get the last read message for a conversation
    const getConversationLastRead = async (
      conversationId: Id<"conversations">,
    ) => {
      const readEntry = await ctx.db
        .query("privateMessageReads")
        .withIndex("by_member_and_conversation", (q) =>
          q.eq("member", currentUser._id).eq("conversation", conversationId),
        )
        .first();
      return readEntry ? readEntry.lastReadMessage : null;
    };

    // Find conversations where currentUser is memberOne
    const conversationsAsMemberOne = await ctx.db
      .query("conversations")
      .withIndex("by_memberOne", (q) => q.eq("memberOne", currentUser._id))
      .collect();

    // Find conversations where currentUser is memberTwo
    const conversationsAsMemberTwo = await ctx.db
      .query("conversations")
      .withIndex("by_memberTwo", (q) => q.eq("memberTwo", currentUser._id))
      .collect();

    const allConversations = [
      ...conversationsAsMemberOne,
      ...conversationsAsMemberTwo,
    ];

    const conversationsWithOtherMember = await Promise.all(
      allConversations.map(async (conversation) => {
        const otherMemberId =
          conversation.memberOne === currentUser._id
            ? conversation.memberTwo
            : conversation.memberOne;

        if (!otherMemberId) {
          return null; // Should not happen for 2-person conversations
        }

        const otherMember = await ctx.db.get(otherMemberId);

        if (!otherMember) {
          return null;
        }

        // Get last message for the conversation
        const lastMessage = await ctx.db
          .query("privateMessages")
          .withIndex("by_conversation", (q) =>
            q.eq("conversation", conversation._id),
          )
          .order("desc")
          .first();

        // Get unread count using the local helper function
        const lastReadMessageId = await getConversationLastRead(
          conversation._id,
        );
        let unreadCount = 0;

        if (lastReadMessageId) {
          const messagesAfterLastRead = await ctx.db
            .query("privateMessages")
            .withIndex("by_conversation", (q) =>
              q.eq("conversation", conversation._id),
            )
            .filter((q) =>
              q.gt(
                q.field("_creationTime"),
                (lastReadMessageId as any)._creationTime,
              ),
            )
            .collect();
          unreadCount = messagesAfterLastRead.length;
        } else if (lastMessage) {
          // If no last read message, all messages are unread
          const allMessages = await ctx.db
            .query("privateMessages")
            .withIndex("by_conversation", (q) =>
              q.eq("conversation", conversation._id),
            )
            .collect();
          unreadCount = allMessages.length;
        }

        return {
          _id: conversation._id,
          otherMember: {
            _id: otherMember._id,
            name: otherMember.name,
            imageUrl: otherMember.image_url,
          },
          lastMessage: lastMessage
            ? {
                content: lastMessage.content,
                _creationTime: lastMessage._creationTime,
              }
            : null,
          unreadCount,
        };
      }),
    );

    return conversationsWithOtherMember.filter(Boolean);
  },
});

