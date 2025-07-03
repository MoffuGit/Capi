import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  servers: defineTable({
    name: v.string(),
    invitations: v.array(v.id("invitations")),
    image_url: v.optional(v.string()),
    owner_id: v.int64(),
  }),
  tasks: defineTable({
    text: v.string(),
  }),
  invitations: defineTable({
    server: v.id("servers"),
    invitation: v.string(),
  }),
  members: defineTable({
    user: v.int64(),
    server: v.id("servers"),
    name: v.string(),
    image_url: v.optional(v.string()),
    status: v.id("userStatus"),
  }).index("by_user", ["user"]),
  userStatus: defineTable({
    user: v.int64(),
    status: v.union(
      v.literal("Online"),
      v.literal("Idle"),
      v.literal("NotDisturb"),
      v.literal("Invisible"),
      v.literal("Offline"),
    ),
  }),
});
