import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  channels: defineTable({
    name: v.string(),
    type: v.union(v.literal("text")),
    server: v.id("servers"),
    category: v.optional(v.id("categories")),
  })
    .index("by_server_and_category", ["server", "category"])
    .index("by_server", ["server"])
    .index("by_category", ["category"]),
  categories: defineTable({
    name: v.string(),
    server: v.id("servers"),
  }).index("by_server", ["server"]),
  servers: defineTable({
    name: v.string(),
    image_url: v.optional(v.string()),
  }),
  users: defineTable({
    authId: v.int64(),
    name: v.string(),
    image_url: v.optional(v.string()),
  }).index("by_auth", ["authId"]),
  tasks: defineTable({
    text: v.string(),
  }),
  invitations: defineTable({
    server: v.id("servers"),
    invitation: v.string(),
  }),
  roles: defineTable({
    server: v.id("servers"),
    name: v.string(),
    isOwner: v.boolean(), // True for the unique owner role
    canBeDeleted: v.boolean(), // False for the owner role
    actions: v.object({
      canManageChannels: v.boolean(),
      canManageCategories: v.boolean(),
      canManageRoles: v.boolean(), // Ability to create, delete, or modify roles
      canManageMembers: v.boolean(), // Ability to kick, ban, or assign roles to members
      canManageServerSettings: v.boolean(),
      canCreateInvitation: v.boolean(),
      // Add more specific actions as needed
    }),
  }).index("by_server", ["server"]), // Index to quickly find roles for a server
  members: defineTable({
    user: v.id("users"),
    server: v.id("servers"),
    roles: v.array(v.id("roles")), // A member can now have multiple roles
    name: v.string(),
    image_url: v.optional(v.string()),
    status: v.id("userStatus"),
  })
    .index("by_user", ["user"])
    .index("by_server", ["server"]), // Index to find members of a server
  userStatus: defineTable({
    user: v.id("users"),
    status: v.union(
      v.literal("Online"),
      v.literal("Idle"),
      v.literal("NotDisturb"),
      v.literal("Invisible"),
      v.literal("Offline"),
    ),
  }),
});
