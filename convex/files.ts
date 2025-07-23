import { mutation } from "./_generated/server";
import { ConvexError, v } from "convex/values";

export const generateUploadUrl = mutation({
  args: {
    auth: v.int64(),
  },
  handler: async (ctx, { auth }) => {
    let user = await ctx.db
      .query("users")
      .withIndex("by_auth", (q) => q.eq("authId", auth))
      .unique();
    if (user === null) {
      throw new ConvexError("You need to be auth to upload a file");
    }
    return await ctx.storage.generateUploadUrl();
  },
});
