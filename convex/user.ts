import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const getServers = query({
  args: {
    user: v.id("users"),
  },
  handler: async (ctx, { user }) => {
    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user))
      .collect();

    const results = await Promise.all(
      members.map(async (member) => {
        const server = await ctx.db.get(member.server);
        return { member, server };
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
    user: v.id("users"),
    serverId: v.id("servers"),
  },
  handler: async (ctx, { user, serverId }) => {
    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", serverId).eq("user", user),
      )
      .unique();

    return member;
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
    await ctx.db.insert("users", { authId: auth, name, image_url });
  },
});
