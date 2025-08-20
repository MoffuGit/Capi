import { mutation, query } from "./_generated/server";
import { Id, Doc } from "./_generated/dataModel.js";
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

// Define the type for _storage metadata
type StorageMetadata = {
  _id: Id<"_storage">;
  _creationTime: number;
  contentType?: string;
  sha256: string;
  size: number;
};

// Define the shape of the additional data fetched for a message
type MessageRelatedData = {
  reactions: Array<
    Doc<"messageReactionCounts"> & {
      hasReacted: boolean;
    }
  >;
  mentions: Array<Doc<"mentions">>;
  role_mentions: Array<Doc<"role_mentions">>;
  attachments: Array<
    Doc<"attachments"> & {
      url: string | null;
      metadata: StorageMetadata | null;
    }
  >;
};

// Define the full message type including related data
type FullMessageType = Doc<"messages"> & MessageRelatedData;

export const getMessagesInChannel = query({
  args: {
    channelId: v.id("channels"),
    memberId: v.id("members"),
  },
  handler: async (ctx, { channelId, memberId }) => {
    const fetchMessageRelatedData = async (
      messageId: Id<"messages">,
    ): Promise<MessageRelatedData> => {
      const [messageReactionCounts, mentions, roleMentions, attachments] =
        await Promise.all([
          ctx.db
            .query("messageReactionCounts")
            .withIndex("by_message_and_emoji", (q) =>
              q.eq("message", messageId),
            )
            .collect(),
          ctx.db
            .query("mentions")
            .withIndex("by_message", (q) => q.eq("message", messageId))
            .collect(),
          ctx.db
            .query("role_mentions")
            .withIndex("by_message", (q) => q.eq("message", messageId))
            .collect(),
          ctx.db
            .query("attachments")
            .withIndex("by_message", (q) => q.eq("message", messageId))
            .collect()
            .then(async (atts) =>
              Promise.all(
                atts.map(async (att) => {
                  const url = await ctx.storage.getUrl(att.storageId);
                  const metadata = (await ctx.db.system.get(
                    att.storageId,
                  )) as StorageMetadata | null;
                  return {
                    ...att, // Include _id, _creationTime, message, storageId from original Doc
                    url,
                    metadata,
                  };
                }),
              ),
            ),
        ]);

      const reactionsWithHasReacted = await Promise.all(
        messageReactionCounts.map(async (reactionCount) => {
          const memberReaction = await ctx.db
            .query("memberReactions")
            .withIndex("by_message_member_emoji", (q) =>
              q
                .eq("message", messageId)
                .eq("member", memberId)
                .eq("emoji", reactionCount.emoji),
            )
            .first();
          return {
            ...reactionCount,
            hasReacted: !!memberReaction,
          };
        }),
      );
      return {
        reactions: reactionsWithHasReacted,
        mentions,
        role_mentions: roleMentions,
        attachments,
      };
    };

    const messages = await ctx.db
      .query("messages")
      .withIndex("by_channel", (q) => q.eq("channel", channelId))
      .collect();

    const fullMessages = await Promise.all(
      messages.map(async (message) => {
        const { reactions, mentions, role_mentions, attachments } =
          await fetchMessageRelatedData(message._id);

        let referencedMessage: FullMessageType | null = null;
        if (message.reference) {
          const refMsg = await ctx.db.get(message.reference);
          if (refMsg) {
            const refData = await fetchMessageRelatedData(refMsg._id);
            referencedMessage = {
              ...refMsg,
              ...refData,
            };
          }
        }

        return {
          ...message,
          reactions,
          mentions,
          role_mentions,
          attachments,
          referencedMessage,
        };
      }),
    );

    return fullMessages;
  },
});

export const addAttachmentToMessage = mutation({
  args: {
    messageId: v.id("messages"),
    storageId: v.id("_storage"),
    name: v.string(),
  },
  handler: async (ctx, args) => {
    const newAttachment = {
      message: args.messageId,
      storageId: args.storageId,
      name: args.name,
    };
    return await ctx.db.insert("attachments", newAttachment);
  },
});

export const deleteMessage = mutation({
  args: {
    messageId: v.id("messages"),
  },
  handler: async (ctx, args) => {
    const memberReactionsToDelete = await ctx.db
      .query("memberReactions")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(memberReactionsToDelete.map((r) => ctx.db.delete(r._id)));

    const messageReactionCountsToDelete = await ctx.db
      .query("messageReactionCounts")
      .filter((q) => q.eq(q.field("message"), args.messageId))
      .collect();
    await Promise.all(
      messageReactionCountsToDelete.map((r) => ctx.db.delete(r._id)),
    );

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
    await Promise.all(
      attachments.map(async (a) => {
        if (a.storageId) {
          await ctx.storage.delete(a.storageId);
        }
        await ctx.db.delete(a._id);
      }),
    );

    return await ctx.db.delete(args.messageId);
  },
});

export const updateMessage = mutation({
  args: {
    messageId: v.id("messages"),
    pinned: v.optional(v.boolean()),
    content: v.optional(v.string()),
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
