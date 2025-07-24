import { v } from "convex/values";
import { mutation, query } from "./_generated/server";
import { ConvexError } from "convex/values";

export const create = mutation({
  args: {
    name: v.string(),
    auth: v.int64(),
    image_url: v.optional(v.string()),
  },
  handler: async ({ db }, { name, image_url, auth }) => {
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const serverId = await db.insert("servers", {
      name: name,
      image_url: image_url,
    });

    // Create the owner role for the new server
    const ownerRole = await db.insert("roles", {
      server: serverId,
      name: "Owner",
      isOwner: true,
      canBeDeleted: false,
      level: 0,
      actions: {
        canManageChannels: true,
        canManageCategories: true,
        canManageRoles: true,
        canManageMembers: true,
        canManageServerSettings: true,
        canCreateInvitation: true,
      },
    });

    // Create the default 'Member' role
    const defaultMemberRole = await db.insert("roles", {
      server: serverId,
      name: "Member",
      isOwner: false,
      canBeDeleted: false, // Should not be deleted
      level: 100, // Lower priority than owner
      actions: {
        canManageChannels: false,
        canManageCategories: false,
        canManageRoles: false,
        canManageMembers: false,
        canManageServerSettings: false,
        canCreateInvitation: true, // Only this capability
      },
    });

    // Update the server to link the default role
    await db.patch(serverId, { defaultRole: defaultMemberRole });

    await db.insert("members", {
      user: user._id,
      server: serverId,
      name: user.name,
      image_url: user.image_url,
      roles: [ownerRole, defaultMemberRole], // Owner gets both roles
      mostImportantRole: ownerRole, // Owner role is still the most important
      online: true,
    });

    return serverId;
  },
});

export const getChannels = query({
  args: {
    server: v.id("servers"),
    category: v.optional(v.id("categories")),
  },
  handler: async ({ db }, { server, category }) => {
    return await db
      .query("channels")
      .withIndex("by_server_and_category", (q) =>
        q.eq("server", server).eq("category", category),
      )
      .collect();
  },
});

export const getCategories = query({
  args: {
    server: v.id("servers"),
  },
  handler: async ({ db }, { server }) => {
    return await db
      .query("categories")
      .withIndex("by_server", (q) => q.eq("server", server))
      .collect();
  },
});
