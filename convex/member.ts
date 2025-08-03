import { v } from "convex/values";
import type { Doc } from "./_generated/dataModel";
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

export const getMembersByIds = query({
  args: {
    memberIds: v.array(v.id("members")),
  },
  handler: async ({ db }, { memberIds }) => {
    const members: Array<Doc<"members">> = [];
    for (const memberId of memberIds) {
      const member = await db.get(memberId);
      if (member) {
        members.push(member);
      }
    }
    return members;
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
