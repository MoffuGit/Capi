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

    const server = await db.insert("servers", {
      name: name,
      image_url: image_url,
    });

    // Create the owner role for the new server
    const ownerRole = await db.insert("roles", {
      server: server,
      name: "Owner",
      isOwner: true,
      canBeDeleted: false,
      actions: {
        canManageChannels: true,
        canManageCategories: true,
        canManageRoles: true,
        canManageMembers: true,
        canManageServerSettings: true,
        canCreateInvitation: true,
      },
    });

    const status = await db.insert("userStatus", {
      user: user._id,
      status: "Online",
    });

    await db.insert("members", {
      user: user._id,
      server: server,
      name: user.name,
      image_url: user.image_url,
      status: status,
      roles: [ownerRole],
    });

    return server;
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
