import Store from "../../../../store/fea_store.js";
import { UPDATE_GLOBAL_MESH_SEED_MESSAGE_HEADER } from "../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER } from "../../../../consts/fea_app_consts.js";


class FeaMeshGlobalMeshSeedMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            store: new Store(),
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: flex;
                flex-direction: column;
            }

            .wrapper {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .global-mesh-seed-buttons {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                align-self: center;
            }

            .apply-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
            }

            .apply-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .nodes-along-line-slider-container {
                display: flex;
                flex-direction: column;
                padding: 0rem;
                margin: 1rem 0rem 0rem 0rem;
            }

            .nodes-along-line-caption {
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                padding: 0rem;
                margin: 0rem;
                width: 10rem;
            }

            .nodes-along-line-slider {
                -webkit-appearance: none;
                appearance: none;
                height: 0.5rem;
                background: var(--preprocessor-menu-content-caption-color);
                outline: none;
                opacity: 0.7;
                -webkit-transition: .2s;
                transition: opacity .2s;
                margin-top: 0.5rem;
            }

            .nodes-along-line-slider:hover {
                opacity: 1;
            }

            .nodes-along-line-slider::-webkit-slider-thumb {
                -webkit-appearance: none;
                appearance: none;
                width: 0.7rem;
                height: 1.1rem;
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                cursor: pointer;
            }

            .nodes-along-line-slider::-moz-range-thumb {
                width: 0.7rem;
                height: 1.1rem;
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                cursor: pointer;
            }
        </style>
        <div class=wrapper>

            <div class="nodes-along-line-slider-container">
                <p class="nodes-along-line-caption">
                    The number of nodes along a line / edge:  
                    <span class="nodes-along-line-span"></span>
                </p>
                <input class="nodes-along-line-slider" type="range" min="1" max="8">
            </div>

            <div class="global-mesh-seed-buttons">
                <button class="apply-button">Apply</button>
            </div>

        </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateGlobalMeshSeed());

        this.shadowRoot.querySelector(".nodes-along-line-slider").addEventListener("input", () => {
            const output = this.shadowRoot.querySelector(".nodes-along-line-span");
            output.innerHTML = parseInt(this.shadowRoot.querySelector(".nodes-along-line-slider").value) + 1;
        });
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        const globalMeshSeed = this.props.store.global_mesh_seed;
        this.shadowRoot.querySelector(".nodes-along-line-span").innerHTML = parseInt(globalMeshSeed) + 1;
        this.shadowRoot.querySelector(".nodes-along-line-slider").value = globalMeshSeed;
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["action-id"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = parseInt(newValue);
            this.defineGlobalMeshSeed();
        }
    }

    adoptedCallback() {
    }

    defineGlobalMeshSeed() {
        this.shadowRoot.querySelector(".nodes-along-line-span").innerHTML = 
            parseInt(this.props.store.global_mesh_seed) + 1;
        this.shadowRoot.querySelector(".nodes-along-line-slider").value = this.props.store.global_mesh_seed;
    }

    updateGlobalMeshSeed() {
        const oldGlobalMeshSeedValue = this.props.store.global_mesh_seed;
        const newGlobalMeshSeedValue = parseInt(this.shadowRoot.querySelector(".nodes-along-line-slider").value);

        const message = { [UPDATE_GLOBAL_MESH_SEED_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            old_global_mesh_seed_value: oldGlobalMeshSeedValue,
            new_global_mesh_seed_value: newGlobalMeshSeedValue,
        }};

        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }
}

export default FeaMeshGlobalMeshSeedMenu;
