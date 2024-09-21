import { createResource, For, Suspense } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Character {
  name: string;
  breed?: string;
  enabled: boolean;
}

interface Group {
  name: string;
  characters: Character[]
}

function App() {
  const [groups, { mutate }] = createResource<Group[]>(async () => (await invoke("get_groups")));
  const [available_characters, { refetch }] = createResource<Character[]>(async () => (await invoke("get_available_characters")));

  return (
    <>
      <div>
        <form onsubmit={async (e: SubmitEvent) => {
          e.preventDefault();
          const form = e.target as HTMLFormElement;
          const data = new FormData(form);
          const groups: Group[] = await invoke("create_group", Object.fromEntries(data));
          mutate(groups);
          form.reset();
        }}>
          <input type="text" name="name" required minlength="3" />
        </form>
      </div>
      <Suspense>
        <For each={groups()}>{(group, i) =>
          <div>
            <label for={group.name}>
              <h2>{group.name}
                <button style="margin-left: 5px" onclick={async () => {
                  const groups: Group[] = await invoke("delete_group", { id: i() });
                  mutate(groups);
                }}>
                  Delete
                </button>
              </h2>
            </label>
            <input type="radio" id={group.name} name="group" value={group.name} checked={i() === 0} />
            <div class="content">

              <h4 ondragover={(e: DragEvent) => e.preventDefault()} ondrop={async (e: DragEvent) => {
                const name = e.dataTransfer?.getData("application/group-available")
                if (!name) {
                  return
                }

                const groups: Group[] = await invoke("add_character_to_group", { id: i(), name })
                mutate(groups)
              }}>Selected</h4>
              <div>
                <For each={group.characters}>
                  {(character, characterId) =>
                    <div title={character.name} style="display: inline-block;cursor: pointer;" draggable ondragstart={(e: DragEvent) => {
                      e.dataTransfer?.setData("application/group-selected", characterId().toString())
                      e.dataTransfer?.setData("application/group-selected-pos", character.name)


                    }} ondragover={(e: DragEvent) => e.preventDefault()} ondrop={async (e: DragEvent) => {
                      const name = e.dataTransfer?.getData("application/group-available") || e.dataTransfer?.getData("application/group-selected-pos")

                      if (!name || name === character.name) {
                        return
                      }

                      let { x, width } = (e.target as HTMLElement).getBoundingClientRect()
                      const middleX = x + width / 2
                      const groups: Group[] = await invoke("add_character_to_group_at", { id: i(), name, targetName: character.name, right: e.clientX > middleX })
                      mutate(groups)

                    }}>
                      <img src={`/breeds/${character.breed || 'None'}.png`} />
                    </div>
                  }
                </For>
              </div>
              <h4 ondragover={(e: DragEvent) => e.preventDefault()} ondrop={async (e: DragEvent) => {
                const characterId = e.dataTransfer?.getData("application/group-selected")
                if (!characterId) {
                  return
                }

                const groups: Group[] = await invoke("remove_character_from_group", { id: i(), characterId: parseInt(characterId, 10) })
                mutate(groups)
              }}>
                Available
                <button style="margin-left: 5px" onclick={() => refetch()}>Refresh</button>
              </h4>
              <div>
                <For each={available_characters()?.filter((ac) => !group.characters.some((c) => c.name === ac.name))}>
                  {(character) =>
                    <div title={character.name} style="display: inline-block; cursor: pointer;" draggable ondragstart={(e: DragEvent) => {
                      e.dataTransfer?.setData("application/group-available", character.name)

                    }}>
                      <img src={`/breeds/${character.breed || 'None'}.png`} />
                    </div>
                  }
                </For>
              </div>
            </div>
          </div>
        }
        </For>
      </Suspense>
    </>
  );
}

export default App;
