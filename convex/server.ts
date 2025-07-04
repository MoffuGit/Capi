import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const create = mutation({
  args: {
    auth: v.int64(),
    name: v.string(),
    image_url: v.optional(v.string()),
  },
  handler: async ({ db }, { auth, name, image_url }) => {
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      return;
    }
    const server = await db.insert("servers", {
      name: name,
      image_url: image_url,
      invitations: [],
    });
    const status = await db.insert("userStatus", {
      user: user._id,
      status: "Online",
    });
    await db.insert("members", {
      user: user._id,
      server: server,
      name: "Default",
      status: status,
    });
  },
});
