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

    const serverIds = members.map((member) => member.server);
    const servers = await Promise.all(
      serverIds.map((serverId) => ctx.db.get(serverId)),
    );

    return servers.filter((q) => q !== null);
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
