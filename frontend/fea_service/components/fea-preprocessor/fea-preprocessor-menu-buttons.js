class FeaPreprocessorMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "geometry-menu-button",
                "material-menu-button",
                "section-menu-button",
                "properties-menu-button",
                "load-menu-button",
                "boundary-condition-menu-button",
                "mesh-menu-button",
                "analysis-menu-button",
            ],

            menuNames: {
                "geometry-menu-button": "geometry-menu",
                "material-menu-button": "material-menu",
                "section-menu-button": "section-menu",
                "properties-menu-button": "properties-menu",
                "load-menu-button": "load-menu",
                "boundary-condition-menu-button": "boundary-condition-menu",
                "mesh-menu-button": "mesh-menu",
                "analysis-menu-button": "analysis-menu",
            }
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: flex;
            }

            .wrapper {
                background-color: var(--preprocessor-menu-buttons-bg-color);
                display: flex;
                flex-direction: column;
                position: relative;
                overflow-y: auto;
                overflow-x: hidden;
                scrollbar-color: var(--renderer-menu-content-scrollbar-thumb-color) var(--preprocessor-menu-buttons-active-button-bg-color);
                scrollbar-width: thin;
            }

            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            ::-webkit-scrollbar-track {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            ::-webkit-scrollbar-thumb {
                background: var(--renderer-menu-content-scrollbar-thumb-color);
            }

            ::-webkit-scrollbar-thumb:hover {
                background: var(--renderer-menu-content-scrollbar-thumb-hover-color);
            }

            .geometry-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .geometry-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .geometry-menu-button:hover .geometry-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-icon-hover-bg-color);
            }

            .active .geometry-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .geometry-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .geometry-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .geometry-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .geometry-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .material-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .material-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .material-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .material-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .material-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .material-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .material-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .section-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .section-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .section-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .section-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .section-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .section-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .section-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .properties-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .properties-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .properties-menu-button:hover .properties-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-icon-hover-bg-color);
            }

            .properties-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .properties-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .properties-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .properties-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .properties-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .load-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .load-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .load-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .load-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .load-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .load-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .load-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .boundary-condition-menu-button {
                margin: 0rem;
                padding-top: 0.325rem;
                padding-bottom: 0.325rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .boundary-condition-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .boundary-condition-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .boundary-condition-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .boundary-condition-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .boundary-condition-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .boundary-condition-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .mesh-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .mesh-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .mesh-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .mesh-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .mesh-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .mesh-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .mesh-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .analysis-menu-button {
                margin: 0rem;
                padding-top: 0.7rem;
                padding-bottom: 0.7rem;
                background: var(--preprocessor-menu-buttons-button-bg-color);
                border: var(--preprocessor-menu-buttons-button-border-color);
            }

            .analysis-menu-button:hover {
                background: var(--preprocessor-menu-buttons-button-hover-bg-color);
            }

            .analysis-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
            }

            .analysis-menu-button-icon {
                color: var(--preprocessor-menu-buttons-button-bg-color);
                width: 3.5rem;
                height: 3.5rem;
            }

            .analysis-menu-button-icon-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin: 0rem;
                padding: 0rem;
                width: 3.5rem;
                font-size: 85%;
            }

            .active .analysis-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-bg-color);
            }

            .active:hover .analysis-menu-button-icon {
                color: var(--preprocessor-menu-buttons-active-button-icon-hover-bg-color);
            }

            .active:hover {
                background: var(--preprocessor-menu-buttons-active-button-hover-bg-color);
            }

            .active {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
            }
        </style>
        <div class="wrapper">
            <button class="geometry-menu-button">
                <div class="geometry-menu-button-icon-content">
                    <svg class=geometry-menu-button-icon width="100" height="100" viewBox="0 0 100 100" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Geometry</title>
                        <g fill="currentColor">
                            <path d="M6.77588 74.5L44.4645 1.0947L82.1532 74.5H6.77588Z" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                            />
                            <rect x="52.5" y="29.5" width="40" height="42" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                            />
                            <circle cx="77.5" cy="65.5" r="22" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                            />
                            <path d="M33.8771 87.5L21.5761 66L33.8771 44.5H58.4554L69.4385 66L58.4554 
                                87.5H33.8771Z" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                            />
                        </g>
                    </svg>
                    <p class="geometry-menu-button-icon-caption">Geometry</p>
                </div>
            </button>

            <button class="material-menu-button">
                <div class="material-menu-button-icon-content">
                    <svg class=material-menu-button-icon width="105" height="104" viewBox="0 0 105 104" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                    <title>Material</title>
                        <path d="M4.35355 6.1885C4.15829 5.99324 3.84171 5.99324 3.64645 6.1885L0.464466 
                            9.37048C0.269204 9.56574 0.269204 9.88232 0.464466 10.0776C0.659728 
                            10.2728 0.976311 10.2728 1.17157 10.0776L4 7.24916L6.82843 10.0776C7.02369 
                            10.2728 7.34027 10.2728 7.53553 10.0776C7.7308 9.88232 7.7308 9.56574 
                            7.53553 9.37048L4.35355 6.1885ZM4.5 100L4.5 6.54205H3.5L3.5 100H4.5Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M104.354 100.354C104.549 100.158 104.549 99.8417 
                            104.354 99.6464L101.172 96.4645C100.976 96.2692 100.66 96.2692 100.464 
                            96.4645C100.269 96.6597 100.269 96.9763 100.464 97.1716L103.293 100L100.464 
                            102.828C100.269 103.024 100.269 103.34 100.464 103.536C100.66 103.731 100.976 
                            103.731 101.172 103.536L104.354 100.354ZM4 100.5H104V99.5H4V100.5Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M5 100L32.5 50C42 36.4486 63.8 27.1028 91 36.4486" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)" stroke-width="2"/>
                        <path d="M35.1094 17.8789H28.9766C30.3958 18.7643 31.4961 19.9427 32.2773 
                            21.4141C33.0716 22.8724 33.4688 24.5586 33.4688 26.4727V27.1172C33.4688 
                            29.0443 33.0781 30.7956 32.2969 32.3711C31.5156 33.9466 30.4089 
                            35.1771 28.9766 36.0625C27.5573 36.9479 25.9492 37.3906 24.1523 
                            37.3906C21.3529 37.3906 19.0872 36.4076 17.3555 34.4414C15.6367 
                            32.4622 14.7773 29.8451 14.7773 26.5898V26.1211C14.7773 23.0872 
                            15.6432 20.6198 17.375 18.7188C19.1198 16.8177 21.3919 15.8672 
                            24.1914 15.8672H35.1094V17.8789ZM17.1211 26.7656C17.1211 29.3307 
                            17.7656 31.4141 19.0547 33.0156C20.3438 34.6172 22.043 35.418 
                            24.1523 35.418C26.2227 35.418 27.9023 34.6237 29.1914 33.0352C30.4935 
                            31.4336 31.1445 29.2786 31.1445 26.5703V26.1211C31.1445 23.7122 30.5 21.7396 
                            29.2109 20.2031C27.9219 18.6536 26.2161 17.8789 24.0938 17.8789C22.0104 
                            17.8789 20.3242 18.6602 19.0352 20.2227C17.7591 21.7721 17.1211 23.7904 
                            17.1211 26.2773V26.7656Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M52.0898 77.9282C52.0898 75.3241 53.5547 73.6444 56.4844 72.8892C55.1953 
                            72.4334 54.1992 71.7954 53.4961 70.9751C52.806 70.1548 52.4609 69.2368 
                            52.4609 68.2212C52.4609 66.3332 53.1966 64.8618 54.668 
                            63.8071C56.1393 62.7524 58.1641 62.2251 60.7422 62.2251C63.151 
                            62.2251 65.1562 62.798 66.7578 63.9438C68.3724 65.0897 69.1797 
                            66.5415 69.1797 68.2993H66.8555C66.8555 67.1144 66.263 66.1379 
                            65.0781 65.3696C63.9062 64.5884 62.4609 64.1978 60.7422 64.1978C58.9193 
                            64.1978 57.474 64.5493 56.4062 65.2524C55.3516 65.9556 54.8242 66.9321 
                            54.8242 68.1821C54.8242 70.7082 56.8229 71.9712 60.8203 
                            71.9712H64.2773V73.9243H60.5273C58.5482 73.9373 57.0378 74.2629 
                            55.9961 74.9009C54.9674 75.5389 54.4531 76.561 54.4531 77.9673C54.4531 
                            79.2303 55.0195 80.2459 56.1523 81.0142C57.2982 81.7694 58.8281 82.147 
                            60.7422 82.147C62.6042 82.147 64.1406 81.7238 65.3516 80.8774C66.5755 
                            80.0311 67.1875 78.9634 67.1875 77.6743H69.5312C69.5312 79.6274 68.7044 
                            81.1899 67.0508 82.3618C65.4102 83.5337 63.3073 84.1196 60.7422 84.1196C58.151 
                            84.1196 56.0612 83.5597 54.4727 82.4399C52.8841 81.3071 52.0898 79.8032 
                            52.0898 77.9282Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                    </svg>
                    <p class="material-menu-button-icon-caption">Material</p>
                </div>
            </button>

            <button class="section-menu-button">
                <div class="section-menu-button-icon-content">
                    <svg class="section-menu-button-icon" width="102" height="101" viewBox="0 0 102 101" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Section</title>
                        <path d="M92.3979 11.101H3.30803V21.202H34.5829C38.725 21.202 42.0829 24.5598 
                            42.0829 28.702V77.8434C42.0829 81.9855 38.725 85.3434 34.5829 
                            85.3434H1V96.9595H92.3979V85.3434H59.2766C55.1344 85.3434 51.7766 81.9855 51.7766 
                            77.8434V28.702C51.7766 24.5598 55.1345 21.202 59.2766 21.202H92.3979V11.101Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M47.5901 0.646447C47.3949 0.451184 47.0783 0.451184 46.883 0.646447L43.701 
                            3.82843C43.5058 4.02369 43.5058 4.34027 43.701 4.53553C43.8963 4.7308 44.2129 
                            4.7308 44.4081 4.53553L47.2366 1.70711L50.065 4.53553C50.2603 4.7308 50.5768 
                            4.7308 50.7721 4.53553C50.9674 4.34027 50.9674 4.02369 50.7721 3.82843L47.5901 
                            0.646447ZM47.7366 101V98.5H46.7366V101H47.7366ZM47.7366 
                            93.5V88.5H46.7366V93.5H47.7366ZM47.7366 83.5V78.5H46.7366V83.5H47.7366ZM47.7366 
                            73.5V68.5H46.7366V73.5H47.7366ZM47.7366 63.5V58.5H46.7366V63.5H47.7366ZM47.7366 
                            53.5V48.5H46.7366V53.5H47.7366ZM47.7366 43.5V38.5H46.7366V43.5H47.7366ZM47.7366 
                            33.5V28.5H46.7366V33.5H47.7366ZM47.7366 23.5V18.5H46.7366V23.5H47.7366ZM47.7366 
                            13.5V8.5H46.7366V13.5H47.7366ZM47.7366 3.5V1H46.7366V3.5H47.7366Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M101.354 51.8586C101.549 51.6633 101.549 51.3467 101.354 51.1515L98.1716 
                            47.9695C97.9763 47.7742 97.6597 47.7742 97.4645 47.9695C97.2692 48.1647 97.2692 
                            48.4813 97.4645 48.6766L100.293 51.505L97.4645 54.3334C97.2692 54.5287 97.2692 
                            54.8453 97.4645 55.0405C97.6597 55.2358 97.9763 55.2358 98.1716 55.0405L101.354 
                            51.8586ZM47.2366 52.005H49.9247V51.005H47.2366V52.005ZM55.3011 
                            52.005H60.6774V51.005H55.3011V52.005ZM66.0538 52.005H71.4301V51.005H66.0538V52.005ZM76.8065 
                            52.005H82.1828V51.005H76.8065V52.005ZM87.5591 52.005H92.9355V51.005H87.5591V52.005ZM98.3118 
                            52.005H101V51.005H98.3118V52.005Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                    </svg>
                    <p class="section-menu-button-icon-caption">Section</p>
                </div>
            </button>

            <button class="properties-menu-button">
                <div class="properties-menu-button-icon-content">
                    <svg class="properties-menu-button-icon" width="101" height="101" viewBox="0 0 101 101" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Properties</title>
                        <g fill="currentColor">
                            <path d="M1 1H52.5625L76 35.375V91.625H1V1Z" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                            />
                            <ellipse cx="66.7609" cy="68.1875" rx="34.2391" ry="32.8125"/>
                            <path d="M8.6087 9.33334V83.2917H37.6472M49.3563 9.33334V34.3333H66.2174" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                            />
                            <path d="M63.0756 39.5417L62.2567 49.3519L56.5241 51.3139L49.1537 45.0354L43.4211 
                                50.5291L49.5631 57.9849L47.5158 63.0862L36.8696 63.871V71.7192L47.1063 72.504L49.5631 
                                77.6053L42.6021 85.0611L48.3347 90.5548L56.1147 84.2763L61.4378 86.6307L62.2567 
                                96.8333H70.4461L71.265 87.0231L77.4071 84.6687L85.5965 91.3396L91.3291 
                                85.8459L83.9586 77.9977L86.4154 72.8964L96.6522 72.1116V64.6558L86.4154 
                                63.871L84.3681 58.3773L91.3291 50.5291L85.5965 45.0354L77.4071 51.7063L72.084 
                                49.7443L71.265 39.5417H63.0756Z" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                            />
                            <ellipse cx="66.7609" cy="68.1875" rx="10.3261" ry="9.89583" 
                                stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                            />
                        </g>
                    </svg>
                    <p class="properties-menu-button-icon-caption">Properties</p>
                </div>
            </button>

            <button class="load-menu-button">
                <div class="load-menu-button-icon-content">
                    <svg class="load-menu-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Load</title>
                        <path d="M85.5 56.7797L78.1388 45.339H92.8612L85.5 56.7797Z" 
                            fill="var(--preprocessor-menu-buttons-button-icon-content-color-2)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M86 24.5763L86 44.9152" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M101 101V73.0339" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)" 
                            stroke-linecap="round" stroke-dasharray="4 4"
                        />
                        <path d="M34.5 57.7797C76 60.322 92.5 64.983 101 73.0339" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)" 
                            stroke-dasharray="4 4"
                        />
                        <path d="M34.5 85.7458C76 88.2881 92.5 92.9491 101 101" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)" 
                            stroke-dasharray="4 4"
                        />
                        <path d="M34.3333 1H1V85.7457H101V57.4971H34.3333V1Z" 
                        stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                    />
                    </svg>
                    <p class="load-menu-button-icon-caption">Load</p>
                </div>
            </button>

            <button class="boundary-condition-menu-button">
                <div class="boundary-condition-menu-button-icon-content">
                    <svg class="boundary-condition-menu-button-icon" width="100" height="101" viewBox="0 0 100 101" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Boundary condition</title>
                        <path d="M35.7143 1H7.14285V93.0761H92.8571V62.3841H35.7143V1Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <line x1="100" y1="94.0761" y2="94.0761" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)" 
                            stroke-width="2"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 2.38095 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 7.14285 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 11.9048 101)"
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 16.6667 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 21.4286 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 26.1905 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 30.9524 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 69.0476 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 73.8095 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 78.5714 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 83.3333 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 88.0952 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 92.8571 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 35.7143 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 40.4762 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 45.2381 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 50 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 54.7619 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 59.5238 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <line y1="-0.5" x2="7.54397" y2="-0.5" transform="matrix(0.473415 -0.880839 0.849826 0.527064 64.2857 101)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                    </svg>
                    <p class="boundary-condition-menu-button-icon-caption">Boundary condition</p>
                </div>
            </button>

            <button class="mesh-menu-button">
                <div class="mesh-menu-button-icon-content">
                    <svg class="mesh-menu-button-icon" width="102" height="102" viewBox="0 0 102 102" 
                        fill="none" xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Mesh</title>
                        <rect x="63.5" y="88.5" width="12.5" height="12.5" rx="0.01" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="63.5" y="76" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="1" y="88.5" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="13.5" y="88.5" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="26" y="88.5" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="38.5" y="88.5" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="51" y="88.5" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="1" y="76" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="13.5" y="76" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="26" y="76" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="38.5" y="76" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="51" y="76" width="12.5" height="12.5" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="13.5" y="38.5" width="12.5" height="12.5" transform="rotate(-90 13.5 38.5)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="1" y="38.5" width="12.5" height="12.5" rx="0.01" transform="rotate(-90 1 38.5)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="13.5" y="76" width="12.5" height="12.5" transform="rotate(-90 13.5 76)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="13.5" y="63.5022" width="12.5" height="12.5" transform="rotate(-90.0103 13.5 63.5022)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="13.5" y="51" width="12.5" height="12.5" transform="rotate(-90 13.5 51)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="1" y="76" width="12.5" height="12.5" transform="rotate(-90 1 76)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="1" y="63.5" width="12.5" height="12.5" transform="rotate(-90 1 63.5)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="1" y="51" width="12.5" height="12.5" transform="rotate(-90 1 51)" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M76 88.5041C76 88.5015 76.0011 88.4989 76.0029 88.4971L84.3163 80.1837C84.3226 80.1774 84.3333 
                            80.1819 84.3333 80.1908V92.6625C84.3333 92.6652 84.3323 92.6677 84.3304 92.6696L76.0171 100.983C76.0108 
                            100.989 76 100.985 76 100.976V88.5041Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M92.6667 59.3375C92.6667 59.3348 92.6677 59.3323 92.6696 59.3304L100.983 51.0171C100.989 51.0108 
                            101 51.0152 101 51.0241V63.4959C101 63.4985 100.999 63.5011 100.997 63.5029L92.6838 71.8163C92.6775 
                            71.8226 92.6667 71.8181 92.6667 71.8092V59.3375Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M92.6667 71.8333L101 63.5V76L92.6667 84.3333V71.8333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M76 76L84.3333 67.6667V80.1667L76 88.5V76Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M26 38.5L34.3333 30.1667V42.6667L26 51V38.5Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M42.6667 9.33748C42.6667 9.33482 42.6677 9.33228 42.6696 9.33041L50.9829 1.01707C50.9892 
                            1.01077 51 1.01523 51 1.02414V13.4959C51 13.4985 50.999 13.5011 50.9971 13.5029L42.6837 21.8163C42.6774 
                            21.8226 42.6667 21.8181 42.6667 21.8092V9.33748Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M42.6667 21.8333L51 13.5V26L42.6667 34.3333V21.8333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M26 26L34.3333 17.6667V30.1667L26 38.5V26Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M26 63.5L34.3333 55.1667V67.6667L26 76V63.5Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M42.6667 34.3333L51 26V38.5L42.6667 46.8333V34.3333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M42.6667 46.8333L51 38.5V51L42.6667 59.3333V46.8333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M26 51L34.3333 42.6667V55.1667L26 63.5V51Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M63.5 76L71.8333 67.6667H84.3333L76 76H63.5Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M80.1908 59.3333C80.1819 59.3333 80.1775 59.3226 80.1838 59.3163L88.4971 51.0029C88.499 
                            51.0011 88.5015 51 88.5042 51H100.976C100.985 51 100.989 51.0108 100.983 51.0171L92.6696 
                            59.3304C92.6677 59.3323 92.6652 59.3333 92.6625 59.3333H80.1908Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M1.02414 26C1.01523 26 1.01077 25.9892 1.01707 25.9829L9.33041 17.6696C9.33228 17.6677 
                            9.33482 17.6667 9.33748 17.6667H21.8092C21.8181 17.6667 21.8226 17.6774 21.8163 17.6837L13.5029 
                            25.9971C13.5011 25.9989 13.4985 26 13.4959 26H1.02414Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M17.6667 9.33333L26 1H38.5L30.1667 9.33333H17.6667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M13.5 26L21.8333 17.6667H34.3333L26 26H13.5Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M30.1908 9.33333C30.1819 9.33333 30.1774 9.32256 30.1837 9.31626L38.4971 1.00293C38.499 
                            1.00105 38.5015 1 38.5041 1H50.9759C50.9848 1 50.9892 1.01077 50.9829 1.01707L42.6696 9.3304C42.6677 
                            9.33228 42.6652 9.33333 42.6625 9.33333H30.1908Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M26 76L34.3333 67.6667H46.8333L38.5 76H26Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M42.6667 59.3333L51 51H63.5L55.1667 59.3333H42.6667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M38.5 76L46.8333 67.6667H59.3333L51 76H38.5Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M55.1667 59.3333L63.5 51H76L67.6667 59.3333H55.1667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M51 76L59.3333 67.6667H71.8333L63.5 76H51Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M67.6667 59.3333L76 51H88.5L80.1667 59.3333H67.6667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M84.3333 67.6667L92.6666 59.3333V71.8333L84.3333 80.1667V67.6667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M84.3333 80.1667L92.6666 71.8333V84.3333L84.3333 92.6667V80.1667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M34.3333 17.6667L42.6667 9.33334V21.8333L34.3333 30.1667V17.6667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M34.3333 30.1667L42.6667 21.8333V34.3333L34.3333 42.6667V30.1667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M34.3333 42.6667L42.6667 34.3333V46.8333L34.3333 55.1667V42.6667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M34.3333 55.1667L42.6667 46.8333V59.3333L34.3333 67.6667V55.1667Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M71.8333 67.6667L80.1667 59.3333H92.6667L84.3333 67.6667H71.8333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M9.33334 17.6667L17.6667 9.33334H30.1667L21.8333 17.6667H9.33334Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M21.8333 17.6667L30.1667 9.33334H42.6667L34.3333 17.6667H21.8333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M34.3333 67.6667L42.6667 59.3333H55.1667L46.8333 67.6667H34.3333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M46.8333 67.6667L55.1667 59.3333H67.6667L59.3333 67.6667H46.8333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M59.3333 67.6667L67.6667 59.3333H80.1667L71.8333 67.6667H59.3333Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                    </svg>
                    <p class="mesh-menu-button-icon-caption">Mesh</p>
                </div>
            </button>

            <button class="analysis-menu-button">
                <div class="analysis-menu-button-icon-content">
                    <svg class="analysis-menu-button-icon" width="102" height="102" viewBox="0 0 102 102" fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <title>Analysis</title>
                        <rect x="12.8577" y="1" width="79.0514" height="60.5263" rx="0.01" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="17.6008" y="6.26318" width="69.5652" height="50" rx="0.01" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <rect x="1" y="82.579" width="82.2134" height="18.4211" rx="0.01" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M18.7836 64.161C18.7855 64.159 18.7881 64.1579 18.7908 64.1579H100.976C100.985 
                            64.1579 100.99 64.1685 100.984 64.1748L83.2164 82.5759C83.2145 82.5778 83.2119 82.579 
                            83.2092 82.579H1.02356C1.01474 82.579 1.01023 82.5684 1.01636 82.562L18.7836 64.161Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M83.2134 82.583C83.2134 82.5804 83.2144 82.5779 83.2162 82.576L100.983 
                            64.1757C100.989 64.1692 101 64.1737 101 64.1827V82.5749C101 82.5775 100.999 
                            82.58 100.997 82.5819L83.2306 100.982C83.2244 100.989 83.2134 100.984 83.2134 
                            100.975V82.583Z" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="16.0198" y="82.579" width="3.95257" height="18.4211" rx="0.01" 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <path d="M83.2134 82.579H80.8519C80.8464 82.579 80.8419 82.5835 80.8419 
                            82.589V100.99C80.8419 100.996 80.8464 101 80.8519 101H83.2134M83.2134 
                            82.579V101M83.2134 82.579L85.5675 79.9667C85.5737 79.9599 85.585 79.9643
                            85.585 79.9734V98.3646C85.585 98.3671 85.5841 98.3694 85.5824 98.3713L83.2134 101"
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-2)"
                        />
                        <rect x="32.6205" y="61.5263" width="5.5336" height="11.4035" rx="0.01" 
                            fill=var(--preprocessor-menu-buttons-button-bg-color) 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <rect x="66.6126" y="61.5263" width="5.5336" height="11.4035" rx="0.01" 
                            fill=var(--preprocessor-menu-buttons-button-bg-color) 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M38.1541 61.5363C38.1541 61.5308 38.1586 61.5263 38.1641 61.5263H40.5157C40.5212 
                            61.5263 40.5257 61.5308 40.5257 61.5363V70.7325C40.5257 70.7352 40.5245 
                            70.7379 40.5225 70.7398L38.1709 72.9143C38.1645 72.9202 38.1541 72.9157 
                            38.1541 72.9069V61.5363Z" 
                            fill=var(--preprocessor-menu-buttons-button-bg-color) 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                        <path d="M72.1462 61.5363C72.1462 61.5308 72.1507 61.5263 72.1562 
                            61.5263H74.5078C74.5133 61.5263 74.5178 61.5308 74.5178 61.5363V70.7325C74.5178 
                            70.7352 74.5166 70.7379 74.5146 70.7398L72.163 72.9143C72.1566 72.9202 72.1462 
                            72.9157 72.1462 72.9069V61.5363Z" 
                            fill=var(--preprocessor-menu-buttons-button-bg-color) 
                            stroke="var(--preprocessor-menu-buttons-button-icon-content-color-1)"
                        />
                    </svg>
                    <p class="analysis-menu-button-icon-caption">Analysis</p>
                </div>
            </button>

        </div>
        `;

        this.shadowRoot.querySelector(".geometry-menu-button").addEventListener("click", () => this.toggle("geometry-menu-button"));

        this.shadowRoot.querySelector(".material-menu-button").addEventListener("click", () => this.toggle("material-menu-button"));

        this.shadowRoot.querySelector(".section-menu-button").addEventListener("click", () => this.toggle("section-menu-button"));

        this.shadowRoot.querySelector(".properties-menu-button").addEventListener("click", () => this.toggle("properties-menu-button"));

        this.shadowRoot.querySelector(".load-menu-button").addEventListener("click", () => this.toggle("load-menu-button"));

        this.shadowRoot.querySelector(".boundary-condition-menu-button").addEventListener("click", 
            () => this.toggle("boundary-condition-menu-button"));

        this.shadowRoot.querySelector(".mesh-menu-button").addEventListener("click", () => this.toggle("mesh-menu-button"));

        this.shadowRoot.querySelector(".analysis-menu-button").addEventListener("click", () => this.toggle("analysis-menu-button"));
    }

    set toggleButton(buttonName) {
        this.toggle(buttonName);
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }
    
    static get observedAttributes() {
        return [];
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
    }
    
    adoptedCallback() {
    }

    toggle(buttonName) {
        for (let i = 0; i < this.state.buttonNames.length; i ++) {
            if (this.state.buttonNames[i] !== buttonName && 
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`)
                    .classList.contains("active") === true) {
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).classList.remove("active");
                const menuName = this.state.menuNames[this.state.buttonNames[i]];
                this.dispatchEvent(new CustomEvent("deactivate-menu", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        "menuName": menuName,
                    }
                }));
            }
        } 
        const currentButton = this.shadowRoot.querySelector(`.${buttonName}`);
        if (currentButton.classList.contains("active") === true) {
            currentButton.classList.remove("active");
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("deactivate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": menuName,
                }
            }));
        } else {
            currentButton.classList.add("active");
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("activate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": menuName,
                }
            }));
        }
    }
}

export default FeaPreprocessorMenuButtons;
