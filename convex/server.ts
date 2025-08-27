import { v } from "convex/values";
import { mutation, query } from "./_generated/server";
import { ConvexError } from "convex/values";
import { Id } from "./_generated/dataModel";
import { api } from "./_generated/api.js";

export const create = mutation({
  args: {
    name: v.string(),
    auth: v.int64(),
    storage: v.optional(v.id("_storage")),
    type: v.union(v.literal("public"), v.literal("private")),
  },
  handler: async (
    { db, storage: storageCtx },
    { name, storage: storageId, auth, type },
  ) => {
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    let newImageUrl: string | undefined = undefined;
    if (storageId) {
      let url = await storageCtx.getUrl(storageId);
      newImageUrl = url ? url : undefined;
    }

    const serverId = await db.insert("servers", {
      name: name,
      image_url: newImageUrl,
      imageId: storageId,
      type,
    });

    const ownerRole = await db.insert("roles", {
      server: serverId,
      name: "Owner",
      isOwner: true,
      canBeDeleted: false,
      level: 0,
      actions: {
        canManageChannels: true,
        canManageCategories: true,
        canManageRoles: true,
        canManageMembers: true,
        canManageServerSettings: true,
        canPinMessages: true,
        canCreateInvitation: true,
      },
    });

    const defaultMemberRole = await db.insert("roles", {
      server: serverId,
      name: "Member",
      isOwner: false,
      canBeDeleted: false,
      level: 100,
      actions: {
        canPinMessages: false,
        canManageChannels: false,
        canManageCategories: false,
        canManageRoles: false,
        canManageMembers: false,
        canManageServerSettings: false,
        canCreateInvitation: true,
      },
    });

    await db.patch(serverId, { defaultRole: defaultMemberRole });

    await db.insert("members", {
      user: user._id,
      server: serverId,
      name: user.name,
      image_url: user.image_url,
      roles: [ownerRole, defaultMemberRole],
      mostImportantRole: ownerRole,
    });

    return serverId;
  },
});

export const getChannels = query({
  args: {
    server: v.id("servers"),
    category: v.optional(v.id("categories")),
  },
  handler: async ({ db }, { server, category }) => {
    return await db
      .query("channels")
      .withIndex("by_server_and_category", (q) =>
        q.eq("server", server).eq("category", category),
      )
      .collect();
  },
});

export const getCategories = query({
  args: {
    server: v.id("servers"),
  },
  handler: async ({ db }, { server }) => {
    return await db
      .query("categories")
      .withIndex("by_server", (q) => q.eq("server", server))
      .collect();
  },
});

export const setServerBannerUrl = mutation({
  args: {
    auth: v.int64(),
    serverId: v.id("servers"),
    storageId: v.id("_storage"),
  },
  handler: async (ctx, { auth, serverId, storageId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user._id),
      )
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );

    const canManageServerSettings = memberRoles.some(
      (role) => role?.actions.canManageServerSettings,
    );

    if (!canManageServerSettings) {
      throw new ConvexError(
        "You do not have permission to manage server settings.",
      );
    }

    const server = await ctx.db.get(serverId);

    if (!server) {
      return null;
    }

    const oldServerBannerId = server.bannerId;

    const newBannerUrl = await ctx.storage.getUrl(storageId);

    if (!newBannerUrl) {
      return null;
    }

    if (oldServerBannerId) {
      await ctx.storage.delete(oldServerBannerId);
    }

    await ctx.db.patch(server._id, {
      bannerUrl: newBannerUrl,
      bannerId: storageId,
    });

    return newBannerUrl;
  },
});

export const setServerImageUrl = mutation({
  args: {
    auth: v.int64(),
    serverId: v.id("servers"),
    storageId: v.id("_storage"),
  },
  handler: async (ctx, { auth, serverId, storageId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user._id),
      )
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );

    const canManageServerSettings = memberRoles.some(
      (role) => role?.actions.canManageServerSettings,
    );

    if (!canManageServerSettings) {
      throw new ConvexError(
        "You do not have permission to manage server settings.",
      );
    }

    const server = await ctx.db.get(serverId);

    if (!server) {
      return null;
    }

    const oldServerImageId = server.imageId;

    const newImageUrl = await ctx.storage.getUrl(storageId);

    if (!newImageUrl) {
      return null;
    }

    if (oldServerImageId) {
      await ctx.storage.delete(oldServerImageId);
    }

    await ctx.db.patch(server._id, {
      image_url: newImageUrl,
      imageId: storageId,
    });

    return newImageUrl;
  },
});

export const updateServerDescription = mutation({
  args: {
    auth: v.int64(),
    serverId: v.id("servers"),
    description: v.string(),
  },
  handler: async (ctx, { auth, serverId, description }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user._id),
      )
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );

    const canManageServerSettings = memberRoles.some(
      (role) => role?.actions.canManageServerSettings,
    );

    if (!canManageServerSettings) {
      throw new ConvexError(
        "You do not have permission to manage server settings.",
      );
    }

    const server = await ctx.db.get(serverId);

    if (!server) {
      throw new ConvexError("Server not found");
    }

    await ctx.db.patch(server._id, { description });
    return true;
  },
});

export const removeServerImage = mutation({
  args: {
    auth: v.int64(),
    serverId: v.id("servers"),
  },
  handler: async (ctx, { auth, serverId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user._id),
      )
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );

    const canManageServerSettings = memberRoles.some(
      (role) => role?.actions.canManageServerSettings,
    );

    if (!canManageServerSettings) {
      throw new ConvexError(
        "You do not have permission to manage server settings.",
      );
    }

    const server = await ctx.db.get(serverId);

    if (!server) {
      return null;
    }

    const oldServerImageId = server.imageId;

    if (oldServerImageId) {
      await ctx.storage.delete(oldServerImageId);
    }

    await ctx.db.patch(server._id, {
      image_url: undefined,
      imageId: undefined,
    });

    return true;
  },
});

export const removeServerBanner = mutation({
  args: {
    auth: v.int64(),
    serverId: v.id("servers"),
  },
  handler: async (ctx, { auth, serverId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user._id),
      )
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );

    const canManageServerSettings = memberRoles.some(
      (role) => role?.actions.canManageServerSettings,
    );

    if (!canManageServerSettings) {
      throw new ConvexError(
        "You do not have permission to manage server settings.",
      );
    }

    const server = await ctx.db.get(serverId);

    if (!server) {
      return null;
    }

    const oldServerBannerId = server.bannerId;

    if (oldServerBannerId) {
      await ctx.storage.delete(oldServerBannerId);
    }

    await ctx.db.patch(server._id, {
      bannerUrl: undefined,
      bannerId: undefined,
    });

    return true;
  },
});

export const getPublicServers = query({
  args: {
    auth: v.int64(),
  },
  handler: async ({ db }, { auth }) => {
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }
    return await db
      .query("servers")
      .filter((q) => q.eq(q.field("type"), "public"))
      .collect();
  },
});

export const joinServer = mutation({
  args: {
    serverId: v.id("servers"),
    userId: v.id("users"),
  },
  handler: async (ctx, { serverId, userId }) => {
    const user = await ctx.db.get(userId);
    if (user === null) {
      throw new Error("User not found.");
    }

    const serverDoc = await ctx.db.get(serverId);
    if (!serverDoc) {
      throw new Error("Server not found.");
    }

    const existingMember = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", userId),
      )
      .first();

    if (existingMember) {
      return existingMember._id;
    }

    let defaultRoleForNewMember: Id<"roles">[] = [];
    let mostImportantRoleForNewMember: Id<"roles"> | undefined = undefined;

    if (serverDoc.defaultRole) {
      defaultRoleForNewMember.push(serverDoc.defaultRole);
      mostImportantRoleForNewMember = serverDoc.defaultRole;
    }

    const newMemberId = await ctx.db.insert("members", {
      user: userId,
      server: serverId,
      roles: defaultRoleForNewMember,
      name: user.name,
      image_url: user.image_url,
      mostImportantRole: mostImportantRoleForNewMember,
    });

    const channelsInServer = await ctx.db
      .query("channels")
      .withIndex("by_server", (q) => q.eq("server", serverId))
      .collect();

    await Promise.all(
      channelsInServer.map((channel) =>
        ctx.runMutation(
          api.unreadMessages.initializeMemberChannelLastReadOnJoin,
          {
            memberId: newMemberId,
            channelId: channel._id,
          },
        ),
      ),
    );
    return newMemberId;
  },
});
