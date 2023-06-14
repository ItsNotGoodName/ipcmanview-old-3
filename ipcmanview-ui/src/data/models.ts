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

export interface Camera {
  created_at: any;
  /** @format int64 */
  id: number;
  ip: string;
  refreshed_at: any;
  username: string;
}

export interface CameraDetail {
  device_class: string;
  device_type: string;
  hardware_version: string;
  market_area: string;
  process_info: string;
  sn: string;
  vendor: string;
}

export interface CameraFile {
  /** @format int64 */
  camera_id: number;
  end_time: any;
  events: string[];
  file_path: string;
  /** @format int64 */
  id: number;
  kind: string;
  /** @format int64 */
  size: number;
  start_time: any;
  updated_at: any;
}

export interface CameraFileQuery {
  after?: string | null;
  before?: string | null;
  camera_ids?: number[];
  end?: null;
  events?: string[];
  kinds?: string[];
  /** @format int32 */
  limit?: number | null;
  start?: null;
}

export interface CameraFileQueryResult {
  after: string;
  before: string;
  /** @format int32 */
  count: number;
  files: CameraFile[];
  has_after: boolean;
  has_before: boolean;
}

export interface CameraFileTotalQuery {
  camera_ids?: number[];
  end?: null;
  events?: string[];
  kinds?: string[];
  start?: null;
}

export interface CameraLicense {
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
  effective_time: any;
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

export interface CameraShow {
  created_at: any;
  detail: CameraDetail;
  /** @format int32 */
  file_total: number;
  /** @format int64 */
  id: number;
  ip: string;
  licenses: CameraLicense[];
  refreshed_at: any;
  software: CameraSoftware;
  username: string;
}

export interface CameraSoftware {
  build: string;
  build_date: string;
  security_base_line_version: string;
  version: string;
  web_version: string;
}

export interface CreateCameraRequest {
  ip: string;
  password: string;
  username: string;
}

export interface DateTimeRange {
  end: any;
  start: any;
}

export interface PageQuery {
  /** @format int32 */
  page?: number;
  /** @format int32 */
  per_page?: number;
}

export interface ScanActive {
  /** @format int64 */
  camera_id: number;
  /** @format int64 */
  deleted: number;
  kind: any;
  /** @format double */
  percent: number;
  range_cursor: any;
  range_end: any;
  range_start: any;
  started_at: any;
  /** @format int64 */
  upserted: number;
}

export interface ScanCompleted {
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
  range_cursor: any;
  range_end: any;
  range_start: any;
  retry_pending: boolean;
  started_at: any;
  success: boolean;
  /** @format int64 */
  upserted: number;
}

export interface ScanCompletedPageResult {
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

export interface ScanPending {
  /** @format int64 */
  camera_id: number;
  /** @format int64 */
  id: number;
  kind: any;
  range_end: any;
  range_start: any;
}

export interface TotalQueryResult {
  /** @format int32 */
  total: number;
}

export interface UpdateCameraRequest {
  /** @format int64 */
  id: number;
  ip?: string | null;
  password?: string | null;
  username?: string | null;
}
