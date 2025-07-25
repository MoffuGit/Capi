import { v } from "convex/values";
import { mutation, query } from "./_generated/server";
import type { Doc } from "./_generated/dataModel";

export const getServers = query({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, { auth }) => {
    let user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      return [];
    }
    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .collect();

    const results = await Promise.all(
      members.map(async (member) => {
        const server = await ctx.db.get(member.server);
        const roles: Doc<"roles">[] = [];
        for (const roleId of member.roles) {
          const role = await ctx.db.get(roleId);
          if (role) {
            roles.push(role);
          }
        }
        return { member, server, roles };
      }),
    );

    return results.filter((item) => item.server !== null);
  },
});

export const getUser = query({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, { auth }) => {
    return await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
  },
});

export const getMemberForServerByUser = query({
  args: {
    auth: v.int64(), // Changed from user to auth
    serverId: v.id("servers"),
  },
  handler: async (ctx, { auth, serverId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (user === null) {
      return null; // User not found for auth ID
    }

    const member = await ctx.db
      .query("members")
      .withIndex(
        "by_server_and_user",
        (q) => q.eq("server", serverId).eq("user", user._id), // Use user._id here
      )
      .unique();

    if (!member) {
      return null; // Member not found
    }

    const roles: Doc<"roles">[] = [];
    for (const roleId of member.roles) {
      const role = await ctx.db.get(roleId);
      if (role) {
        roles.push(role);
      }
    }

    return { member, roles }; // Return member and its roles
  },
});

export const setLastVisitedChannel = mutation({
  args: {
    auth: v.int64(),
    member: v.id("members"),
    channel: v.id("channels"),
  },
  handler: async (ctx, { auth, member, channel }) => {
    let user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      return;
    }

    const memberData = await ctx.db.get(member);
    if (memberData === null || memberData.user !== user._id) {
      return;
    }

    await ctx.db.patch(member, { lastVisitedChannel: channel });
  },
});

export const getMembers = query({
  args: {
    user: v.id("users"),
  },
  handler: async (ctx, { user }) => {
    return await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user))
      .collect();
  },
});

export const create = mutation({
  args: {
    auth: v.int64(),
    name: v.string(),
    image_url: v.optional(v.string()),
  },
  handler: async (ctx, { auth, name, image_url }) => {
    let user = await ctx.db.insert("users", { authId: auth, name, image_url });
    await ctx.db.insert("userStatus", {
      user: user,
      status: "Online",
    });
  },
});

export const setBannerUrl = mutation({
  args: {
    auth: v.int64(),
    storageId: v.id("_storage"),
  },
  handler: async (ctx, { auth, storageId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      return null;
    }

    const oldUserBannerId = user.bannerId; // Store the old bannerId from the user

    const newBannerUrl = await ctx.storage.getUrl(storageId);

    if (!newBannerUrl) {
      return null;
    }

    if (user.bannerId) {
      await ctx.storage.delete(user.bannerId);
    }

    await ctx.db.patch(user._id, {
      bannerUrl: newBannerUrl,
      bannerId: storageId,
    });

    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .collect();

    await Promise.all(
      members.map(async (member) => {
        // Update member's banner if it was previously linked to the user's old banner,
        // or if the member did not have a bannerId set.
        if (
          member.bannerId === oldUserBannerId ||
          member.bannerId === null ||
          member.bannerId === undefined
        ) {
          await ctx.db.patch(member._id, {
            bannerUrl: newBannerUrl,
            bannerId: storageId,
          });
        }
      }),
    );

    return newBannerUrl;
  },
});

export const setImageUrl = mutation({
  args: {
    auth: v.int64(),
    storageId: v.id("_storage"),
  },
  handler: async (ctx, { auth, storageId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      return null;
    }

    const oldUserImageId = user.imageId; // Store the old imageId from the user

    const newImageUrl = await ctx.storage.getUrl(storageId);

    if (!newImageUrl) {
      return null;
    }

    if (user.imageId) {
      await ctx.storage.delete(user.imageId);
    }

    await ctx.db.patch(user._id, {
      image_url: newImageUrl,
      imageId: storageId,
    });

    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .collect();

    await Promise.all(
      members.map(async (member) => {
        // Update member's image if it was previously linked to the user's old image,
        // or if the member did not have an imageId set.
        if (
          member.imageId === oldUserImageId ||
          member.imageId === null ||
          member.imageId === undefined
        ) {
          await ctx.db.patch(member._id, {
            image_url: newImageUrl,
            imageId: storageId,
          });
        }
      }),
    );

    return newImageUrl;
  },
});

export const removeUserImage = mutation({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, { auth }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      return null;
    }

    const oldUserImageId = user.imageId;

    if (oldUserImageId) {
      await ctx.storage.delete(oldUserImageId);
    }

    await ctx.db.patch(user._id, {
      image_url: undefined,
      imageId: undefined,
    });

    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .collect();

    await Promise.all(
      members.map(async (member) => {
        if (member.imageId === oldUserImageId) {
          await ctx.db.patch(member._id, {
            image_url: undefined,
            imageId: undefined,
          });
        }
      }),
    );

    return true;
  },
});

export const removeUserBanner = mutation({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, { auth }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      return null;
    }

    const oldUserBannerId = user.bannerId;

    if (oldUserBannerId) {
      await ctx.storage.delete(oldUserBannerId);
    }

    await ctx.db.patch(user._id, {
      bannerUrl: undefined,
      bannerId: undefined,
    });

    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .collect();

    await Promise.all(
      members.map(async (member) => {
        if (member.bannerId === oldUserBannerId) {
          await ctx.db.patch(member._id, {
            bannerUrl: undefined,
            bannerId: undefined,
          });
        }
      }),
    );

    return true;
  },
});
