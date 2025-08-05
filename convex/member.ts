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
  handler: async ({ db }, { auth, channel, member }) => {
    let user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      return;
    }

    const channelData = await db.get(channel);
    if (channelData === null) {
      return;
    }

    const memberData = await db.get(member);

    if (memberData === null) {
      return;
    }

    const memberId = memberData._id;

    const existingLastVisited = await db
      .query("lastVisitedChannels")
      .withIndex("by_member", (q) => q.eq("member", memberId))
      .unique();

    if (existingLastVisited) {
      await db.patch(existingLastVisited._id, { channel: channel });
    } else {
      await db.insert("lastVisitedChannels", {
        member: memberId,
        channel: channel,
      });
    }
  },
});

export const getLastVisitedChannel = query({
  args: {
    member: v.id("members"),
    auth: v.int64(),
  },
  handler: async ({ db }, { member, auth }) => {
    let user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      return;
    }

    const memberData = await db.get(member);
    if (memberData === null || memberData.user !== user._id) {
      return;
    }
    const lastVisited = await db
      .query("lastVisitedChannels")
      .withIndex("by_member", (q) => q.eq("member", member))
      .unique();

    return lastVisited ? lastVisited.channel : null;
  },
});
