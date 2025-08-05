import { v } from "convex/values";
import { mutation, query } from "./_generated/server";
import { ConvexError } from "convex/values";

export const get = query({
  args: {
    channelId: v.id("channels"),
    serverId: v.id("servers"),
    auth: v.int64(), // Changed from memberId to auth
  },
  handler: async ({ db }, { channelId, serverId, auth }) => {
    const channel = await db.get(channelId);

    if (!channel) {
      throw new ConvexError("Channel not found");
    }

    if (channel.server !== serverId) {
      throw new ConvexError("Channel does not belong to the specified server");
    }

    // Authenticate user by authId
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    // Check if user is a member of the specified server
    const member = await db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user._id),
      )
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    return channel;
  },
});

export const create = mutation({
  args: {
    auth: v.int64(),
    server: v.id("servers"),
    category: v.optional(v.id("categories")),
    name: v.string(),
  },
  handler: async ({ db }, { server, category, name, auth }) => {
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .filter((q) => q.eq(q.field("server"), server))
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => db.get(roleId)),
    );

    const canManageChannels = memberRoles.some(
      (role) => role?.actions.canManageChannels,
    );

    if (!canManageChannels) {
      throw new ConvexError("You do not have permission to manage channels.");
    }

    return await db.insert("channels", {
      name,
      type: "text",
      server,
      category,
    });
  },
});
