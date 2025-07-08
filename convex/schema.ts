import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

const presenceStatus = v.union(
  v.literal("Online"),
  v.literal("Idle"),
  v.literal("NotDisturb"),
  v.literal("Invisible"),
);

export default defineSchema({
  channels: defineTable({
    name: v.string(),
    type: v.union(v.literal("text")),
    server: v.id("servers"),
    topic: v.optional(v.string()),
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
    level: v.number(), // New field to define the importance level of a role
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
    lastVisitedChannel: v.optional(v.id("channels")),
    online: v.boolean(),
    mostImportantRole: v.optional(v.id("roles")), // New field for the most important role
  })
    .index("by_user", ["user"])
    .index("by_server", ["server"])
    .index("by_server_and_user", ["server", "user"])
    .index("by_server_and_important_role", ["server", "mostImportantRole"])
    .index("by_server_and_status", ["server", "online"])
    .index("by_server_and_important_role_and_status", [
      "server",
      "mostImportantRole",
      "online",
    ]),
  userStatus: defineTable({
    user: v.id("users"),
    status: presenceStatus,
  }).index("by_user", ["user"]),
  sessions: defineTable({
    userId: v.id("users"),
    sessionId: v.string(),
  })
    .index("by_sessionId", ["sessionId"])
    .index("by_userId", ["userId"]),

  sessionTimeouts: defineTable({
    sessionId: v.string(),
    scheduledFunctionId: v.id("_scheduled_functions"),
  }).index("by_sessionId", ["sessionId"]),
});
