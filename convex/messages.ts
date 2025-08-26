import { mutation, query } from "./_generated/server";
import { Id, Doc } from "./_generated/dataModel.js";
import { v } from "convex/values";
import { api } from "./_generated/api.js";

import { ConvexError } from "convex/values";
import type { QueryCtx } from "./_generated/server";

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
      mention_everyone: args.mention_everyone ?? false,
      mention_roles: args.mention_roles ?? [],
    };
    const messageId = await ctx.db.insert("messages", newMessage);

    await ctx.runMutation(api.unreadMessages.updateMemberChannelLastRead, {
      memberId: args.senderId,
      channelId: args.channelId,
      messageId: messageId,
    });

    return messageId;
  },
});

type StorageMetadata = {
  _id: Id<"_storage">;
  _creationTime: number;
  contentType?: string;
  sha256: string;
  size: number;
};

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
  pinned: boolean;
};

type FullMessageType = Doc<"messages"> & MessageRelatedData;

async function getFullMessageDetails(
  ctx: QueryCtx,
  message: Doc<"messages">,
  channelId: Id<"channels">,
  memberId: Id<"members">,
): Promise<FullMessageType> {
  const [reactions, mentions, role_mentions, attachments, isPinnedDoc] =
    await Promise.all([
      ctx.db
        .query("messageReactionCounts")
        .withIndex("by_message_and_emoji", (q) => q.eq("message", message._id))
        .collect()
        .then(async (messageReactionCounts) =>
          Promise.all(
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
                hasReacted: !!memberReaction,
              };
            }),
          ),
        ),
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
        .collect()
        .then(async (atts) =>
          Promise.all(
            atts.map(async (att) => {
              const url = await ctx.storage.getUrl(att.storageId);
              const metadata = (await ctx.db.system.get(
                att.storageId,
              )) as StorageMetadata | null;
              return {
                ...att,
                url,
                metadata,
              };
            }),
          ),
        ),
      ctx.db
        .query("pinnedMessages")
        .withIndex("by_message", (q) => q.eq("message", message._id))
        .unique(),
    ]);

  let referencedMessage: FullMessageType | null = null;
  if (message.reference) {
    const refMsg = await ctx.db.get(message.reference);
    if (refMsg) {
      referencedMessage = await getFullMessageDetails(
        ctx,
        refMsg,
        channelId,
        memberId,
      );
    }
  }

  return {
    ...message,
    reactions,
    mentions,
    role_mentions,
    attachments,
    pinned: !!isPinnedDoc,
  };
}

export const getMessagesInChannel = query({
  args: {
    channelId: v.id("channels"),
    memberId: v.id("members"),
  },
  handler: async (ctx, { channelId, memberId }) => {
    const messages = await ctx.db
      .query("messages")
      .withIndex("by_channel", (q) => q.eq("channel", channelId))
      .collect();

    const fullMessages = await Promise.all(
      messages.map((message) =>
        getFullMessageDetails(ctx, message, channelId, memberId),
      ),
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

export const pinMessage = mutation({
  args: {
    auth: v.int64(),
    messageId: v.id("messages"),
    channelId: v.id("channels"),
  },
  handler: async (ctx, { auth, messageId, channelId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (!user) {
      throw new ConvexError("User not found");
    }

    const channel = await ctx.db.get(channelId);
    if (!channel) {
      throw new ConvexError("Channel not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", channel.server).eq("user", user._id),
      )
      .unique();
    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );
    const canManageChannels = memberRoles.some(
      (role) => role?.actions.canManageChannels,
    );
    if (!canManageChannels) {
      throw new ConvexError("You do not have permission to manage channels.");
    }

    const message = await ctx.db.get(messageId);
    if (!message || message.channel !== channelId) {
      throw new ConvexError("Message not found or does not belong to channel");
    }

    const existingPin = await ctx.db
      .query("pinnedMessages")
      .withIndex("by_message", (q) => q.eq("message", messageId))
      .unique();

    if (existingPin) {
      return existingPin._id;
    }

    return await ctx.db.insert("pinnedMessages", {
      message: messageId,
      channel: channelId,
    });
  },
});

export const unpinMessage = mutation({
  args: {
    auth: v.int64(),
    messageId: v.id("messages"),
    channelId: v.id("channels"),
  },
  handler: async (ctx, { auth, messageId, channelId }) => {
    const user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (!user) {
      throw new ConvexError("User not found");
    }

    const channel = await ctx.db.get(channelId);
    if (!channel) {
      throw new ConvexError("Channel not found");
    }

    const member = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", channel.server).eq("user", user._id),
      )
      .unique();
    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => ctx.db.get(roleId)),
    );
    const canManageChannels = memberRoles.some(
      (role) => role?.actions.canManageChannels,
    );
    if (!canManageChannels) {
      throw new ConvexError("You do not have permission to manage channels.");
    }

    const pinnedMessageToDelete = await ctx.db
      .query("pinnedMessages")
      .withIndex("by_message", (q) => q.eq("message", messageId))
      .first();

    if (pinnedMessageToDelete) {
      await ctx.db.delete(pinnedMessageToDelete._id);
      return true;
    }

    return false;
  },
});

export const getPinnedMessages = query({
  args: {
    channelId: v.id("channels"),
    memberId: v.id("members"),
  },
  handler: async (ctx, { channelId, memberId }) => {
    const pinnedMessageEntries = await ctx.db
      .query("pinnedMessages")
      .withIndex("by_channel", (q) => q.eq("channel", channelId))
      .collect();

    const fullPinnedMessages = await Promise.all(
      pinnedMessageEntries.map(async (pinnedEntry) => {
        const message = await ctx.db.get(pinnedEntry.message);
        if (message) {
          return await getFullMessageDetails(ctx, message, channelId, memberId);
        }
        return null;
      }),
    );

    return fullPinnedMessages.filter(
      (message): message is FullMessageType => message !== null,
    );
  },
});
