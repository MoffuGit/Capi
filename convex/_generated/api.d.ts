/* eslint-disable */
/**
 * Generated `api` utility.
 *
 * THIS CODE IS AUTOMATICALLY GENERATED.
 *
 * To regenerate, run `npx convex dev`.
 * @module
 */

import type {
  ApiFromModules,
  FilterApi,
  FunctionReference,
} from "convex/server";
import type * as category from "../category.js";
import type * as channel from "../channel.js";
import type * as invitations from "../invitations.js";
import type * as member from "../member.js";
import type * as presence from "../presence.js";
import type * as roles from "../roles.js";
import type * as server from "../server.js";
import type * as task from "../task.js";
import type * as user from "../user.js";

/**
 * A utility for referencing Convex functions in your app's API.
 *
 * Usage:
 * ```js
 * const myFunctionReference = api.myModule.myFunction;
 * ```
 */
declare const fullApi: ApiFromModules<{
  category: typeof category;
  channel: typeof channel;
  invitations: typeof invitations;
  member: typeof member;
  presence: typeof presence;
  roles: typeof roles;
  server: typeof server;
  task: typeof task;
  user: typeof user;
}>;
export declare const api: FilterApi<
  typeof fullApi,
  FunctionReference<any, "public">
>;
export declare const internal: FilterApi<
  typeof fullApi,
  FunctionReference<any, "internal">
>;
