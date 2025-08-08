import { v } from "convex/values";
import { mutation, query, QueryCtx, MutationCtx } from "./_generated/server.js";
import { api } from "./_generated/api.js";
import { Id } from "./_generated/dataModel.js";

const presenceStatus = v.union(
  v.literal("Online"),
  v.literal("Idle"),
  v.literal("NotDisturb"),
  v.literal("Invisible"),
);
type PresenceStatus = typeof presenceStatus.type;

export async function getUserStatus(
  ctx: QueryCtx | MutationCtx,
  userId: Id<"users">,
) {
  return await ctx.db
    .query("userStatus")
    .withIndex("by_user", (q) => q.eq("user", userId))
    .unique();
}

async function updateMembersOnlineStatus(
  ctx: MutationCtx,
  userId: Id<"users">,
  onlineStatus: boolean,
) {
  const membersToUpdate = await ctx.db
    .query("members")
    .withIndex("by_user", (q) => q.eq("user", userId))
    .collect();

  for (const member of membersToUpdate) {
    if (member.online !== onlineStatus) {
      await ctx.db.patch(member._id, { online: onlineStatus });
    }
  }
}

export const heartbeat = mutation({
  args: {
    user: v.id("users"),
    sessionId: v.string(),
    interval: v.optional(v.number()),
  },
  handler: async (ctx, { user, sessionId, interval = 10000 }) => {
    let sessionRecord = await ctx.db
      .query("sessions")
      .withIndex("by_sessionId", (q) => q.eq("sessionId", sessionId))
      .unique();

    if (!sessionRecord) {
      await ctx.db.insert("sessions", { userId: user, sessionId });
    }

    const userStatusDoc = await getUserStatus(ctx, user);
    const chosenStatus: PresenceStatus = userStatusDoc
      ? userStatusDoc.status
      : "Online";

    const membersShouldBeOnline = chosenStatus !== "Invisible";
    await updateMembersOnlineStatus(ctx, user, membersShouldBeOnline);

    const existingTimeout = await ctx.db
      .query("sessionTimeouts")
      .withIndex("by_sessionId", (q) => q.eq("sessionId", sessionId))
      .unique();

    if (existingTimeout) {
      await ctx.scheduler.cancel(existingTimeout.scheduledFunctionId);
      await ctx.db.delete(existingTimeout._id);
    }

    const timeoutScheduledId = await ctx.scheduler.runAfter(
      interval * 2.5,
      api.presence.disconnectSession,
      { userId: user, sessionId },
    );

    await ctx.db.insert("sessionTimeouts", {
      sessionId,
      scheduledFunctionId: timeoutScheduledId,
    });
  },
});

export const disconnectSession = mutation({
  args: { userId: v.id("users"), sessionId: v.string() },
  handler: async (ctx, { userId, sessionId }) => {
    const sessionRecord = await ctx.db
      .query("sessions")
      .withIndex("by_sessionId", (q) => q.eq("sessionId", sessionId))
      .unique();

    if (sessionRecord) {
      await ctx.db.delete(sessionRecord._id);
    }

    const existingTimeout = await ctx.db
      .query("sessionTimeouts")
      .withIndex("by_sessionId", (q) => q.eq("sessionId", sessionId))
      .unique();

    if (existingTimeout) {
      await ctx.db.delete(existingTimeout._id);
    }

    const remainingSessions = await ctx.db
      .query("sessions")
      .withIndex("by_userId", (q) => q.eq("userId", userId))
      .collect();

    if (remainingSessions.length === 0) {
      await updateMembersOnlineStatus(ctx, userId, false);
    }
  },
});

export const getStatus = query({
  args: { userId: v.id("users") },
  handler: async (ctx, { userId }) => {
    const userStatusDoc = await getUserStatus(ctx, userId);
    return userStatusDoc ? userStatusDoc.status : "Invisible"; // Default to "Invisible" if no record
  },
});

export const patchUserStatus = mutation({
  args: {
    auth: v.int64(),
    status: presenceStatus,
  },
  handler: async (ctx, { auth, status }) => {
    let user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      return;
    }
    const userStatusDoc = await getUserStatus(ctx, user._id);

    if (userStatusDoc) {
      await ctx.db.patch(userStatusDoc._id, { status });
    } else {
      await ctx.db.insert("userStatus", { user: user._id, status });
    }

    const membersShouldBeOnline = status !== "Invisible";
    await updateMembersOnlineStatus(ctx, user._id, membersShouldBeOnline);
  },
});
