import { v } from "convex/values";
import { mutation, query } from "./_generated/server";
import { ConvexError } from "convex/values";

export const create = mutation({
  args: {
    auth: v.int64(),
    server: v.id("servers"),
    name: v.string(),
  },
  handler: async ({ db }, { server, name, auth }) => {
    const user = await db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();

    if (!user) {
      throw new ConvexError("User not found");
    }

    const member = await db
      .query("members")
      .withIndex("by_user", (q) => q.eq("user", user._id))
      .filter((q) => q.eq(q.field("server"), server))
      .unique();

    if (!member) {
      throw new ConvexError("Member not found in this server");
    }

    const memberRoles = await Promise.all(
      member.roles.map((roleId) => db.get(roleId)),
    );

    const canManageCategories = memberRoles.some(
      (role) => role?.actions.canManageCategories,
    );

    if (!canManageCategories) {
      throw new ConvexError("You do not have permission to manage categories.");
    }

    return await db.insert("categories", {
      name,
      server,
    });
  },
});
