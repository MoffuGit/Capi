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
  messages: defineTable({
    channel: v.id("channels"),
    sender: v.id("members"),
    reference: v.optional(v.id("messages")),
    content: v.string(),
    pinned: v.boolean(),
    mention_everyone: v.boolean(),
    mention_roles: v.array(v.id("roles")),
  }).index("by_channel", ["channel"]),
  // embeds: defineTable({
  //     message: v.id("messages"),
  //     url: v.string(),
  // }),
  reactions: defineTable({
    message: v.id("messages"),
    member: v.id("members"),
    name: v.string(),
  }).index("by_message", ["message"]),
  mentions: defineTable({
    message: v.id("messages"),
    member: v.id("member"),
  }).index("by_message", ["message"]),
  role_mentions: defineTable({
    message: v.id("messages"),
    role: v.id("roles"),
  }).index("by_message", ["message"]),
  attachments: defineTable({
    message: v.id("messages"),
    name: v.string(),
    type: v.string(),
    url: v.string(),
  }).index("by_message", ["message"]),
  categories: defineTable({
    name: v.string(),
    server: v.id("servers"),
  }).index("by_server", ["server"]),
  servers: defineTable({
    name: v.string(),
    image_url: v.optional(v.string()),
    defaultRole: v.optional(v.id("roles")),
  }),
  users: defineTable({
    authId: v.int64(),
    name: v.string(),
    image_url: v.optional(v.string()),
    about: v.optional(v.string()),
    bannerUrl: v.optional(v.string()),
  }).index("by_auth", ["authId"]),
  tasks: defineTable({
    text: v.string(),
  }),
  invitations: defineTable({
    server: v.id("servers"),
    invitation: v.string(),
    expiresAt: v.number(),
  })
    .index("by_invitation", ["invitation"])
    .index("by_server", ["server"]),
  roles: defineTable({
    server: v.id("servers"),
    name: v.string(),
    isOwner: v.boolean(),
    canBeDeleted: v.boolean(),
    level: v.number(),
    actions: v.object({
      canManageChannels: v.boolean(),
      canManageCategories: v.boolean(),
      canManageRoles: v.boolean(),
      canManageMembers: v.boolean(),
      canManageServerSettings: v.boolean(),
      canCreateInvitation: v.boolean(),
    }),
  }).index("by_server", ["server"]),
  members: defineTable({
    user: v.id("users"),
    server: v.id("servers"),
    roles: v.array(v.id("roles")),
    name: v.string(),
    image_url: v.optional(v.string()),
    lastVisitedChannel: v.optional(v.id("channels")),
    online: v.boolean(),
    mostImportantRole: v.optional(v.id("roles")),
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
