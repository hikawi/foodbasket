export interface StaffMember {
  id: string;
  name: string;
  avatar_url: string | null;
  pin_code?: string;
  tenant_id: string;
  user_id: string;
  created_at: string;
  updated_at: string;
}

export interface StaffApiResponse {
  data: StaffMember[];
  page: number;
  perPage: number;
  total: number;
  totalPages: number;
}
