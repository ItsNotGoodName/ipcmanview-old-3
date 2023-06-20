/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

export type Camera = {
  /** @format date-time */
  created_at: Date | string;
  /** @format int64 */
  id: number;
  ip: string;
  /** @format date-time */
  refreshed_at: Date | string;
  username: string;
}

export type CameraDetail = {
  device_class: string;
  device_type: string;
  hardware_version: string;
  market_area: string;
  process_info: string;
  sn: string;
  vendor: string;
}

export type CameraFile = {
  /** @format int64 */
  camera_id: number;
  /** @format date-time */
  end_time: Date | string;
  events: string[];
  file_path: string;
  /** @format int64 */
  id: number;
  kind: string;
  /** @format int64 */
  size: number;
  /** @format date-time */
  start_time: Date | string;
  /** @format date-time */
  updated_at: Date | string;
}

export type CameraFileQuery = {
  after?: string | null;
  before?: string | null;
  camera_ids?: number[];
  /** @format date-time */
  end?: Date | string | null;
  events?: string[];
  kinds?: string[];
  /** @format int32 */
  limit?: number | null;
  /** @format date-time */
  start?: Date | string | null;
}

export type CameraFileQueryResult = {
  after: string;
  before: string;
  /** @format int32 */
  count: number;
  files: CameraFile[];
  has_after: boolean;
  has_before: boolean;
}

export type CameraFileTotalQuery = {
  camera_ids?: number[];
  /** @format date-time */
  end?: Date | string | null;
  events?: string[];
  kinds?: string[];
  /** @format date-time */
  start?: Date | string | null;
}

export type CameraLicense = {
  abroad_info: string;
  all_type: boolean;
  /**
   * @format int32
   * @min 0
   */
  digit_channel: number;
  /**
   * @format int32
   * @min 0
   */
  effective_days: number;
  /** @format date-time */
  effective_time: Date | string;
  /**
   * @format int32
   * @min 0
   */
  license_id: number;
  product_type: string;
  /**
   * @format int32
   * @min 0
   */
  status: number;
  username: string;
}

export type CameraShow = {
  /** @format date-time */
  created_at: Date | string;
  detail: CameraDetail;
  /** @format int32 */
  file_total: number;
  /** @format int64 */
  id: number;
  ip: string;
  licenses: CameraLicense[];
  /** @format date-time */
  refreshed_at: Date | string;
  software: CameraSoftware;
  username: string;
}

export type CameraSoftware = {
  build: string;
  build_date: string;
  security_base_line_version: string;
  version: string;
  web_version: string;
}

export type CreateCameraRequest = {
  ip: string;
  password: string;
  username: string;
}

export type DateTimeRange = {
  /** @format date-time */
  end: Date | string;
  /** @format date-time */
  start: Date | string;
}

export type PageQuery = {
  /** @format int32 */
  page?: number;
  /** @format int32 */
  per_page?: number;
}

export type ScanActive = {
  /** @format int64 */
  camera_id: number;
  /** @format int64 */
  deleted: number;
  kind: any;
  /** @format double */
  percent: number;
  /** @format date-time */
  range_cursor: Date | string;
  /** @format date-time */
  range_end: Date | string;
  /** @format date-time */
  range_start: Date | string;
  /** @format date-time */
  started_at: Date | string;
  /** @format int64 */
  upserted: number;
}

export type ScanCompleted = {
  /** @format int64 */
  camera_id: number;
  can_retry: boolean;
  /** @format int64 */
  deleted: number;
  /** @format int64 */
  duration: number;
  error: string;
  /** @format int64 */
  id: number;
  kind: any;
  /** @format double */
  percent: number;
  /** @format date-time */
  range_cursor: Date | string;
  /** @format date-time */
  range_end: Date | string;
  /** @format date-time */
  range_start: Date | string;
  retry_pending: boolean;
  /** @format date-time */
  started_at: Date | string;
  success: boolean;
  /** @format int64 */
  upserted: number;
}

export type ScanCompletedPageResult = {
  items: ScanCompleted[];
  /** @format int32 */
  page: number;
  /** @format int32 */
  per_page: number;
  /** @format int32 */
  total_items: number;
  /** @format int32 */
  total_pages: number;
}

export type ScanPending = {
  /** @format int64 */
  camera_id: number;
  /** @format int64 */
  id: number;
  kind: any;
  /** @format date-time */
  range_end: Date | string;
  /** @format date-time */
  range_start: Date | string;
}

export type TotalQueryResult = {
  /** @format int32 */
  total: number;
}

export type UpdateCameraRequest = {
  /** @format int64 */
  id: number;
  ip?: string | null;
  password?: string | null;
  username?: string | null;
}
