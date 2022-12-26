import { createResource, For, Show } from 'solid-js'
import { Pet } from './api'
import { shopApi } from './signals'

export default function PetList() {
  const [pets, { refetch }] = createResource<Pet[] | undefined>(
    async (k, info) => {
      return await shopApi()?.getPets()
    }
  )

  function buyPet(id: number) {
    shopApi()
      ?.buyPet(id)
      .then(() => {
        refetch()
      })
      .catch((e) => {
        alert('buy pet failed')
      })
  }

  return (
    <div>
      <h3>PetList</h3>
      <Show when={pets()} fallback={<p>fetching pets</p>} keyed>
        {(p) => (
          <For each={p} fallback={<p>all pets are sold</p>}>
            {(item) => {
              return (
                <article>
                  <h4>{item.name}</h4>
                  <p>{item.age}yrs</p>
                  <button onClick={() => buyPet(item.id)}>Buy</button>
                </article>
              )
            }}
          </For>
        )}
      </Show>
    </div>
  )
}
