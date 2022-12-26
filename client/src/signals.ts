import { createSignal } from 'solid-js'
import { AuthV1Api, ShopV1Api } from './api'

export const [authApi, setAuthApi] = createSignal(new AuthV1Api())
export const [shopApi, setShopApi] = createSignal<ShopV1Api>()
