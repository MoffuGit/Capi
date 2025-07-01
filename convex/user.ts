import { v } from "convex/values";
import { query } from "./_generated/server";

export const getServers = query({
  args: {
    userId: v.int64(),
  },
  handler: async (ctx, { userId }) => {
    const members = await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", userId))
      .collect();

    const serverIds = members.map((member) => member.server);
    const servers = await Promise.all(
      serverIds.map((serverId) => ctx.db.get(serverId)),
    );

    return servers.filter(Boolean); // Filter out any nulls if a server wasn't found
  },
});

export const getMembers = query({
  args: {
    id: v.int64(),
  },
  handler: async (ctx, { id }) => {
    return await ctx.db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", id))
      .collect();
  },
});
