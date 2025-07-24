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
