export type Camera = {
  id: number;
  ip: string;
  username: string;
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
