import { query, mutation } from "./_generated/server";
import { v } from "convex/values";

export const getMemberEmojis = query({
  args: {
    memberId: v.id("members"),
  },
  handler: async (ctx, args) => {
    const memberReactions = await ctx.db
      .query("memberReactions")
      .withIndex("by_member", (q) => q.eq("member", args.memberId))
      .collect();

    const uniqueEmojis = new Set<string>();
    for (const reaction of memberReactions) {
      uniqueEmojis.add(reaction.emoji);
    }

    return Array.from(uniqueEmojis);
  },
});

export const addReaction = mutation({
  args: {
    messageId: v.id("messages"),
    memberId: v.id("members"),
    emoji: v.string(),
  },
  handler: async (ctx, { messageId, memberId, emoji }) => {
    const existingMemberReaction = await ctx.db
      .query("memberReactions")
      .withIndex("by_message_member_emoji", (q) =>
        q.eq("message", messageId).eq("member", memberId).eq("emoji", emoji),
      )
      .first();

    if (existingMemberReaction) {
      return { success: false, reason: "Already reacted with this emoji" };
    }

    await ctx.db.insert("memberReactions", {
      message: messageId,
      member: memberId,
      emoji: emoji,
    });

    const existingReactionCount = await ctx.db
      .query("messageReactionCounts")
      .withIndex("by_message_and_emoji", (q) =>
        q.eq("message", messageId).eq("emoji", emoji),
      )
      .first();

    if (existingReactionCount) {
      await ctx.db.patch(existingReactionCount._id, {
        count: existingReactionCount.count + 1,
      });
    } else {
      await ctx.db.insert("messageReactionCounts", {
        message: messageId,
        emoji: emoji,
        count: 1,
      });
    }

    return { success: true };
  },
});

export const removeReaction = mutation({
  args: {
    messageId: v.id("messages"),
    memberId: v.id("members"),
    emoji: v.string(),
  },
  handler: async (ctx, { messageId, memberId, emoji }) => {
    const memberReactionToDelete = await ctx.db
      .query("memberReactions")
      .withIndex("by_message_member_emoji", (q) =>
        q.eq("message", messageId).eq("member", memberId).eq("emoji", emoji),
      )
      .first();

    if (!memberReactionToDelete) {
      return { success: false, reason: "Reaction not found" };
    }

    await ctx.db.delete(memberReactionToDelete._id);

    const existingReactionCount = await ctx.db
      .query("messageReactionCounts")
      .withIndex("by_message_and_emoji", (q) =>
        q.eq("message", messageId).eq("emoji", emoji),
      )
      .first();

    if (existingReactionCount) {
      if (existingReactionCount.count - 1 <= 0) {
        await ctx.db.delete(existingReactionCount._id);
      } else {
        await ctx.db.patch(existingReactionCount._id, {
          count: existingReactionCount.count - 1,
        });
      }
    }

    return { success: true };
  },
});
