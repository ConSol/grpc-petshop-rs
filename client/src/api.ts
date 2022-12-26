import { grpc } from '@improbable-eng/grpc-web'
import {
  AuthenticationClientImpl,
  GrpcWebError,
  GrpcWebImpl,
} from './generated/proto/auth.v1'
import { Pet, PetShopClientImpl } from './generated/proto/shop.v1'

export { Pet } from './generated/proto/shop.v1'

const HOST = 'http://localhost:8081'

export class AuthV1Api {
  private rpc: GrpcWebImpl
  private client: AuthenticationClientImpl

  constructor(usertoken?: string) {
    let metadata: grpc.Metadata | undefined = undefined
    if (usertoken !== undefined) {
      metadata = new grpc.Metadata()
      metadata.append('authentication', `Bearer ${usertoken}`)
    }

    this.rpc = new GrpcWebImpl(HOST, { metadata })
    this.client = new AuthenticationClientImpl(this.rpc)
  }

  async login(email: string, password: string) {
    const { usertoken } = await this.client.Login({ email, password })
    return { usertoken }
  }

  async logout() {
    await this.client.Logout({})
  }
}

export class ShopV1Api {
  private rpc: GrpcWebImpl
  private client: PetShopClientImpl

  constructor(usertoken: string) {
    const metadata = new grpc.Metadata()
    metadata.append('authentication', `Bearer ${usertoken}`)

    this.rpc = new GrpcWebImpl(HOST, { metadata })
    this.client = new PetShopClientImpl(this.rpc)
  }

  async getPets(): Promise<Pet[]> {
    const { pets } = await this.client.GetPets({})
    return pets
  }

  async buyPet(id: number) {
    await this.client.BuyPet({ id })
  }
}
