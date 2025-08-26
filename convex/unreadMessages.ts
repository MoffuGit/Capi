import { mutation, query } from "./_generated/server";
import { v } from "convex/values";
import { Id } from "./_generated/dataModel";

export const initializeMemberChannelLastReadOnJoin = mutation({
  args: {
    memberId: v.id("members"),
    channelId: v.id("channels"),
  },
  handler: async (ctx, { memberId, channelId }) => {
    const existingLastRead = await ctx.db
      .query("memberChannelLastReads")
      .withIndex("by_member_and_channel", (q) =>
        q.eq("member", memberId).eq("channel", channelId),
      )
      .unique();

    if (existingLastRead) {
      await ctx.db.patch(existingLastRead._id, {
        lastReadMessageId: undefined,
      });
    } else {
      await ctx.db.insert("memberChannelLastReads", {
        member: memberId,
        channel: channelId,
        lastReadMessageId: undefined,
      });
    }
  },
});

export const updateMemberChannelLastRead = mutation({
  args: {
    memberId: v.id("members"),
    channelId: v.id("channels"),
    messageId: v.id("messages"),
  },
  handler: async (ctx, { memberId, channelId, messageId }) => {
    const existingLastRead = await ctx.db
      .query("memberChannelLastReads")
      .withIndex("by_member_and_channel", (q) =>
        q.eq("member", memberId).eq("channel", channelId),
      )
      .unique();

    if (existingLastRead) {
      await ctx.db.patch(existingLastRead._id, {
        lastReadMessageId: messageId,
      });
    } else {
      await ctx.db.insert("memberChannelLastReads", {
        member: memberId,
        channel: channelId,
        lastReadMessageId: messageId,
      });
    }
  },
});

export const getUnreadMessagesCountInChannel = query({
  args: {
    memberId: v.id("members"),
    channelId: v.id("channels"),
  },
  handler: async (ctx, { memberId, channelId }) => {
    const memberLastRead = await ctx.db
      .query("memberChannelLastReads")
      .withIndex("by_member_and_channel", (q) =>
        q.eq("member", memberId).eq("channel", channelId),
      )
      .unique();

    if (!memberLastRead || !memberLastRead.lastReadMessageId) {
      const allMessages = await ctx.db
        .query("messages")
        .withIndex("by_channel", (q) => q.eq("channel", channelId))
        .collect();
      return allMessages.length;
    }

    const lastReadMessage = await ctx.db.get(memberLastRead.lastReadMessageId);

    if (!lastReadMessage) {
      const allMessages = await ctx.db
        .query("messages")
        .withIndex("by_channel", (q) => q.eq("channel", channelId))
        .collect();
      return allMessages.length;
    }

    const unreadMessages = await ctx.db
      .query("messages")
      .withIndex("by_channel", (q) =>
        q
          .eq("channel", channelId)
          .gt("_creationTime", lastReadMessage._creationTime),
      )
      .collect();

    return unreadMessages.length;
  },
});

export const getUnreadCountsForAllChannelsForMember = query({
  args: {
    memberId: v.id("members"),
  },
  handler: async (ctx, { memberId }) => {
    const memberChannelReads = await ctx.db
      .query("memberChannelLastReads")
      .withIndex("by_member", (q) => q.eq("member", memberId))
      .collect();

    const unreadCounts: Record<Id<"channels">, number> = {};

    await Promise.all(
      memberChannelReads.map(async (readState) => {
        let unreadCount = 0;
        if (!readState.lastReadMessageId) {
          const allMessages = await ctx.db
            .query("messages")
            .withIndex("by_channel", (q) => q.eq("channel", readState.channel))
            .collect();
          unreadCount = allMessages.length;
        } else {
          const lastReadMessage = await ctx.db.get(readState.lastReadMessageId);

          if (lastReadMessage) {
            const unread = await ctx.db
              .query("messages")
              .withIndex("by_channel", (q) =>
                q
                  .eq("channel", readState.channel)
                  .gt("_creationTime", lastReadMessage._creationTime),
              )
              .collect();
            unreadCount = unread.length;
          } else {
            const allMessages = await ctx.db
              .query("messages")
              .withIndex("by_channel", (q) =>
                q.eq("channel", readState.channel),
              )
              .collect();
            unreadCount = allMessages.length;
          }
        }
        unreadCounts[readState.channel] = unreadCount;
      }),
    );

    return unreadCounts;
  },
});

export const getLastReadMessageId = query({
  args: {
    memberId: v.id("members"),
    channelId: v.id("channels"),
  },
  handler: async (ctx, { memberId, channelId }) => {
    const memberLastRead = await ctx.db
      .query("memberChannelLastReads")
      .withIndex("by_member_and_channel", (q) =>
        q.eq("member", memberId).eq("channel", channelId),
      )
      .unique();
    if (!memberLastRead || !memberLastRead?.lastReadMessageId) {
      return null;
    }
    let message = await ctx.db.get(memberLastRead.lastReadMessageId);

    return message;
  },
});
