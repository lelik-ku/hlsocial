

export interface User {
  user_id?: number,
  first_name?: string | null,
  second_name?: string | null,
  email?: string,
  gender?: string | null,
  birthdate?: number | null,
  biography?: string | null,
  city?: string | null,
}

export interface UserLoginByEmail {
  email?: string,
  passwd?: string,
}

export interface UserPwhash {
  user_id?: number,
  pwhash?: String,
}

export interface UserCreate {
  first_name?: string | null,
  second_name?: string | null,
  email?: string,
  passwd?: string,
  gender?: string | null,
  birthdate?: number | null,
  biography?: string | null,
  city?: string | null,
}

export interface UserUpdate {
  user_id?: number,
  first_name?: string | null,
  second_name?: string | null,
  email?: string,
  gender?: string | null,
  birthdate?: number | null,
  biography?: string | null,
  city?: string | null,
}

export interface UserCreateResult {
  user_id?: number,
}

export interface UserLoginResult {
  user_id?: number,
}
