import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const getOnlineMembersByRole = query({
  args: {
    role: v.optional(v.id("roles")),
    server: v.id("servers"),
  },
  handler: async ({ db }, { role, server }) => {
    return await db
      .query("members")
      .withIndex("by_server_and_important_role_and_status", (q) =>
        q.eq("server", server).eq("mostImportantRole", role).eq("online", true),
      )
      .collect();
  },
});

export const getOfflineMembers = query({
  args: {
    server: v.id("servers"),
  },
  handler: async ({ db }, { server }) => {
    return await db
      .query("members")
      .withIndex("by_server_and_status", (q) =>
        q.eq("server", server).eq("online", false),
      )
      .collect();
  },
});
