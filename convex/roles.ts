import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const serverRoles = query({
  args: {
    server: v.id("servers"),
  },
  handler: async ({ db }, { server }) => {
    return await db
      .query("roles")
      .withIndex("by_server", (q) => q.eq("server", server))
      .collect();
  },
});
