export const INSTANCE_NAME = process.env.PRODUCTION
  ? 'learning-pathways'
  : 'test-instance';
export const ZOME_NAME = 'courses';
export const HOST_URL = `ws://localhost:${process.env.HC_PORT} || 8888`;
export const USERNAME = undefined;