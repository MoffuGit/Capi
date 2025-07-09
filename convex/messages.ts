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
  },
  handler: async (ctx, { channelId }) => {
    const messages = await ctx.db
      .query("messages")
      .withIndex("by_channel", (q) => q.eq("channel", channelId))
      .collect();

    const fullMessages = await Promise.all(
      messages.map(async (message) => {
        const [reactions, mentions, roleMentions, attachments] =
          await Promise.all([
            ctx.db
              .query("reactions")
              .withIndex("by_message", (q) => q.eq("message", message._id))
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

        return {
          ...message,
          reactions,
          mentions,
          role_mentions: roleMentions,
          attachments,
        };
      }),
    );

    return fullMessages;
  },
});

export const deleteMessage = mutation({
  args: {
    messageId: v.id("messages"),
  },
  handler: async (ctx, args) => {
    // Delete reactions, mentions, role_mentions, and attachments associated with the message
    const reactions = await ctx.db
      .query("reactions")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(reactions.map((r) => ctx.db.delete(r._id)));

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
