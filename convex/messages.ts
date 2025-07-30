import { mutation, query } from "./_generated/server";
import { v } from "convex/values";

export const createMessage = mutation({
  args: {
    channelId: v.id("channels"),
    senderId: v.id("members"),
    content: v.string(),
    referenceId: v.optional(v.id("messages")),
    pinned: v.optional(v.boolean()),
    mention_everyone: v.optional(v.boolean()),
    mention_roles: v.optional(v.array(v.id("roles"))),
  },
  handler: async (ctx, args) => {
    const newMessage = {
      channel: args.channelId,
      sender: args.senderId,
      content: args.content,
      reference: args.referenceId,
      pinned: args.pinned ?? false,
      mention_everyone: args.mention_everyone ?? false,
      mention_roles: args.mention_roles ?? [],
    };
    return await ctx.db.insert("messages", newMessage);
  },
});

export const getMessagesInChannel = query({
  args: {
    channelId: v.id("channels"),
    memberId: v.id("members"), // Added memberId argument
  },
  handler: async (ctx, { channelId, memberId }) => {
    // Destructure memberId
    const messages = await ctx.db
      .query("messages")
      .withIndex("by_channel", (q) => q.eq("channel", channelId))
      .collect();

    const fullMessages = await Promise.all(
      messages.map(async (message) => {
        // Fetch aggregated reaction counts for the message
        const [messageReactionCounts, mentions, roleMentions, attachments] =
          await Promise.all([
            ctx.db
              .query("messageReactionCounts")
              .withIndex("by_message_and_emoji", (q) =>
                q.eq("message", message._id),
              )
              .collect(),
            ctx.db
              .query("mentions")
              .withIndex("by_message", (q) => q.eq("message", message._id))
              .collect(),
            ctx.db
              .query("role_mentions")
              .withIndex("by_message", (q) => q.eq("message", message._id))
              .collect(),
            ctx.db
              .query("attachments")
              .withIndex("by_message", (q) => q.eq("message", message._id))
              .collect(),
          ]);

        // For each reaction count, determine if the specified member has reacted
        const reactionsWithHasReacted = await Promise.all(
          messageReactionCounts.map(async (reactionCount) => {
            const memberReaction = await ctx.db
              .query("memberReactions")
              .withIndex("by_message_member_emoji", (q) =>
                q
                  .eq("message", message._id)
                  .eq("member", memberId)
                  .eq("emoji", reactionCount.emoji),
              )
              .first();
            return {
              ...reactionCount,
              hasReacted: !!memberReaction, // true if memberReaction exists, false otherwise
            };
          }),
        );

        return {
          ...message,
          reactions: reactionsWithHasReacted, // Now includes hasReacted property
          mentions,
          role_mentions: roleMentions,
          attachments,
        };
      }),
    );

    return fullMessages;
  },
});

export const addAttachmentToMessage = mutation({
  args: {
    messageId: v.id("messages"),
    name: v.string(),
    type: v.string(),
    url: v.string(),
  },
  handler: async (ctx, args) => {
    const newAttachment = {
      message: args.messageId,
      name: args.name,
      type: args.type,
      url: args.url,
    };
    return await ctx.db.insert("attachments", newAttachment);
  },
});

export const deleteMessage = mutation({
  args: {
    messageId: v.id("messages"),
  },
  handler: async (ctx, args) => {
    // Delete individual member reactions associated with the message
    const memberReactionsToDelete = await ctx.db
      .query("memberReactions")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(memberReactionsToDelete.map((r) => ctx.db.delete(r._id)));

    // Delete aggregated message reaction counts associated with the message
    const messageReactionCountsToDelete = await ctx.db
      .query("messageReactionCounts")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(
      messageReactionCountsToDelete.map((r) => ctx.db.delete(r._id)),
    );

    // Delete mentions, role_mentions, and attachments associated with the message
    const mentions = await ctx.db
      .query("mentions")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(mentions.map((m) => ctx.db.delete(m._id)));

    const roleMentions = await ctx.db
      .query("role_mentions")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(roleMentions.map((rm) => ctx.db.delete(rm._id)));

    const attachments = await ctx.db
      .query("attachments")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(attachments.map((a) => ctx.db.delete(a._id)));

    return await ctx.db.delete(args.messageId);
  },
});

export const updateMessage = mutation({
  args: {
    messageId: v.id("messages"),
    pinned: v.optional(v.boolean()),
    content: v.optional(v.string()), // Allow updating content
  },
  handler: async (ctx, args) => {
    const updateFields: { pinned?: boolean; content?: string } = {};
    if (args.pinned !== undefined) {
      updateFields.pinned = args.pinned;
    }
    if (args.content !== undefined) {
      updateFields.content = args.content;
    }
    return await ctx.db.patch(args.messageId, updateFields);
  },
});

export const addReaction = mutation({
  args: {
    messageId: v.id("messages"),
    memberId: v.id("members"),
    emoji: v.string(),
  },
  handler: async (ctx, { messageId, memberId, emoji }) => {
    // Check if the member has already reacted with this emoji to this message
    const existingMemberReaction = await ctx.db
      .query("memberReactions")
      .withIndex("by_message_member_emoji", (q) =>
        q.eq("message", messageId).eq("member", memberId).eq("emoji", emoji),
      )
      .first();

    if (existingMemberReaction) {
      // Member has already reacted with this emoji, do nothing
      return { success: false, reason: "Already reacted with this emoji" };
    }

    // Add the member's individual reaction
    await ctx.db.insert("memberReactions", {
      message: messageId,
      member: memberId,
      emoji: emoji,
    });

    // Update the aggregated reaction count for the message
    const existingReactionCount = await ctx.db
      .query("messageReactionCounts")
      .withIndex("by_message_and_emoji", (q) =>
        q.eq("message", messageId).eq("emoji", emoji),
      )
      .first();

    if (existingReactionCount) {
      // Increment existing count
      await ctx.db.patch(existingReactionCount._id, {
        count: existingReactionCount.count + 1,
      });
    } else {
      // Create new count entry
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
    // Find and delete the member's individual reaction
    const memberReactionToDelete = await ctx.db
      .query("memberReactions")
      .withIndex("by_message_member_emoji", (q) =>
        q.eq("message", messageId).eq("member", memberId).eq("emoji", emoji),
      )
      .first();

    if (!memberReactionToDelete) {
      // No such reaction found, nothing to remove
      return { success: false, reason: "Reaction not found" };
    }

    await ctx.db.delete(memberReactionToDelete._id);

    // Update the aggregated reaction count for the message
    const existingReactionCount = await ctx.db
      .query("messageReactionCounts")
      .withIndex("by_message_and_emoji", (q) =>
        q.eq("message", messageId).eq("emoji", emoji),
      )
      .first();

    if (existingReactionCount) {
      if (existingReactionCount.count - 1 <= 0) {
        // If count becomes 0 or less, delete the aggregated entry
        await ctx.db.delete(existingReactionCount._id);
      } else {
        // Decrement existing count
        await ctx.db.patch(existingReactionCount._id, {
          count: existingReactionCount.count - 1,
        });
      }
    }
    // If somehow existingReactionCount doesn't exist but memberReaction did,
    // we consider the individual reaction deletion successful and don't error out.

    return { success: true };
  },
});
