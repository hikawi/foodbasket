export const errorCode = {
  unauthorized: "UNAUTHORIZED",
  unknownTenant: "UNKNOWN_TENANT",
} as const;

export type ErrorCode = (typeof errorCode)[keyof typeof errorCode];

export interface ErrorResponse {
  code: ErrorCode;
  message: string;
}
