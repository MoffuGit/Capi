import { mutation, query } from "./_generated/server";
import { v } from "convex/values";
import { Id } from "./_generated/dataModel.js";

export const createInvitation = mutation({
  args: {
    server: v.id("servers"),
    member: v.id("members"),
    expiresInMinutes: v.number(), // Duration until the invitation expires in minutes
  },
  handler: async (ctx, { server, member, expiresInMinutes }) => {
    const m = await ctx.db.get(member);
    if (m === null || m.server !== server) {
      return;
    }

    const now = Date.now();
    const expiresAt = now + expiresInMinutes * 60 * 1000; // Convert minutes to milliseconds

    let invitationCode = crypto.randomUUID();
    let isUnique = false;
    while (!isUnique) {
      const existingInvitation = await ctx.db
        .query("invitations")
        .withIndex("by_invitation", (q) => q.eq("invitation", invitationCode))
        .first();
      if (!existingInvitation) {
        isUnique = true;
      } else {
        invitationCode = crypto.randomUUID();
      }
    }

    const invitationId = await ctx.db.insert("invitations", {
      server: server,
      invitation: invitationCode,
      expiresAt: expiresAt,
    });

    return { invitationCode, invitationId };
  },
});

/**
 * Validates an invitation code.
 * Returns true if the invitation exists and is not expired, false otherwise.
 */
export const validateInvitation = query({
  args: {
    invitationCode: v.string(),
  },
  handler: async (ctx, args) => {
    const invitation = await ctx.db
      .query("invitations")
      .withIndex("by_invitation", (q) =>
        q.eq("invitation", args.invitationCode),
      )
      .first();

    if (!invitation) {
      return false; // Invitation not found
    }

    const now = Date.now();
    if (invitation.expiresAt <= now) {
      return false; // Invitation has expired
    }

    return true; // Invitation is valid
  },
});

/**
 * Retrieves the details of a valid invitation code.
 * Returns the invitation document if valid, otherwise null.
 */
export const getInvitation = query({
  args: {
    invitationCode: v.string(),
  },
  handler: async (ctx, args) => {
    const invitation = await ctx.db
      .query("invitations")
      .withIndex("by_invitation", (q) =>
        q.eq("invitation", args.invitationCode),
      )
      .first();

    if (!invitation) {
      return null; // Invitation not found
    }

    const now = Date.now();
    if (invitation.expiresAt <= now) {
      return null; // Invitation has expired
    }

    return invitation; // Return the valid invitation document
  },
});

/**
 * Deletes an invitation by its ID.
 * Optional: Add permission checks to ensure only authorized users can delete.
 */
export const deleteInvitation = mutation({
  args: {
    invitationId: v.id("invitations"),
  },
  handler: async (ctx, args) => {
    // Optional: Add permission checks here (e.g., only server owner/admin can delete)
    const invitation = await ctx.db.get(args.invitationId);
    if (!invitation) {
      throw new Error("Invitation not found.");
    }
    await ctx.db.delete(args.invitationId);
  },
});

export const joinServerWithInvitation = mutation({
  args: {
    invitationCode: v.string(),
    userId: v.id("users"),
  },
  handler: async (ctx, { invitationCode, userId }) => {
    const invitation = await ctx.db
      .query("invitations")
      .withIndex("by_invitation", (q) => q.eq("invitation", invitationCode))
      .first();

    if (!invitation || invitation.expiresAt <= Date.now()) {
      return null; // Invitation not found or expired
    }

    const user = await ctx.db.get(userId);
    if (user === null) {
      throw new Error("User not found.");
    }

    const existingMember = await ctx.db
      .query("members")
      .withIndex("by_server_and_user", (q) =>
        q.eq("server", invitation.server).eq("user", userId),
      )
      .first();

    if (existingMember) {
      return existingMember._id;
    }

    // Get the server to find its default role
    const serverDoc = await ctx.db.get(invitation.server);
    if (!serverDoc) {
      throw new Error("Server not found for invitation.");
    }

    let defaultRoleForNewMember: Id<"roles">[] = [];
    let mostImportantRoleForNewMember: Id<"roles"> | undefined = undefined;

    if (serverDoc.defaultRole) {
      defaultRoleForNewMember.push(serverDoc.defaultRole);
      // If there's a default role, it's initially the most important for a new member
      mostImportantRoleForNewMember = serverDoc.defaultRole;
    }

    const newMemberId = await ctx.db.insert("members", {
      user: userId,
      server: invitation.server,
      roles: defaultRoleForNewMember,
      name: user.name,
      image_url: user.image_url,
      online: true,
      mostImportantRole: mostImportantRoleForNewMember,
    });

    await ctx.db.delete(invitation._id);

    return newMemberId;
  },
});
