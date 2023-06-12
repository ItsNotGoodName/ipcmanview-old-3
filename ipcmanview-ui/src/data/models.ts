export type PageResult<T> = {
  page: number;
  per_page: number;
  total_pages: number;
  total_items: number;
  items: T;
};

export type CreateCameraMutation = {
  ip: string;
  username: string;
  password: string;
};

export type UpdateCameraMutation = {
  id: number;
  ip: string;
  username: string;
  password: string;
};

export type Camera = {
  id: number;
  ip: string;
  username: string;
};

export type CamerasTotal = {
  total: number;
};

export type ScanPending = {
  id: number;
  camera_id: number;
  range_start: Date;
  range_end: Date;
  kind: string;
};

export type ScanActive = {
  camera_id: number;
  kind: string;
  range_start: Date;
  range_end: Date;
  started_at: Date;
  range_cursor: Date;
  percent: number;
  upserted: number;
  deleted: number;
};

export type ScanCompleted = {
  id: number;
  camera_id: number;
  kind: string;
  range_start: Date;
  range_end: Date;
  started_at: Date;
  range_cursor: Date;
  duration: number;
  error: string;
  percent: number;
  upserted: number;
  deleted: number;
  success: boolean;
  retry_pending: boolean;
  can_retry: boolean;
};

export type ShowCamera = Camera & {
  file_total: number;
  detail: CameraDetail;
  software: CameraSoftware;
  licenses: Array<CameraLicense>;
};

export type CameraDetail = {
  sn: string;
  device_class: string;
  device_type: string;
  hardware_version: string;
  market_area: string;
  process_info: string;
  vendor: string;
};

export type CameraSoftware = {
  build: string;
  build_date: string;
  security_base_line_version: string;
  version: string;
  web_version: string;
};

export type CameraLicense = {
  abroad_info: string;
  all_type: boolean;
  digit_channel: number;
  effective_days: number;
  effective_time: Date;
  license_id: number;
  product_type: string;
  status: number;
  username: string;
};

export type File = {
  id: number;
  camera_id: number;
  file_path: string;
  kind: string;
  start_time: string;
  end_time: string;
  update_at: string;
  events: Array<string>;
};

export type FilesFilter = {
  start?: Date;
  end?: Date;
  kinds?: Array<string>;
  events?: Array<string>;
  camera_ids?: Array<number>;
};

export type InfiniteFilesQuery = {
  limit?: number;
};

export type FilesQuery = {
  limit?: number;
  before?: string;
  after?: string;
};

export type FilesResult = {
  has_before: boolean;
  before: string;
  has_after: boolean;
  after: string;
  files: Array<File>;
};
