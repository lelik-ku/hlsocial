export type PaginatedResponse = {
  total: number
  page: number
}
  
export type PaginatedRequest = {
  limit: number
  page: number
}

export type PaginationParameters = PaginatedResponse & PaginatedRequest

export type PaginationStoreChunk = {
  pagination: PaginationParameters
}

export const DefaultPageSize = 10
