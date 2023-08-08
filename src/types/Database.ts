export type DatabaseEntry = {
  uuid: string;
  name: string;
  url: string;
  username: string;
  password: string;
  notes?: string;
  creation: number;
};

export type Database = {
  version: string;
  root: {
    entries: DatabaseEntry[];
  };
};
