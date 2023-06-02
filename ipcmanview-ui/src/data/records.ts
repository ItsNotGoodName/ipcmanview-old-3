export type PbError = {
  code: number;
  message: string;
  data: {
    [string: string]: Omit<PbError, "data">;
  };
};

export type PbAuth = {
  token: string;
  model: UserRecord | null;
  isValid: boolean;
};

export type UserRecord = {
  avatar: string;
  collectionId: string;
  collectionName: string;
  created: string;
  email: string;
  emailVisibility: boolean;
  id: string;
  name: string;
  updated: string;
  username: string;
  verified: boolean;
};

export type StationRecord = {
  id: string;
  url: string;
  name: string;
};
