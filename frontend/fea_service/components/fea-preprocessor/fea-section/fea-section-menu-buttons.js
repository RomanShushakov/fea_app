class FeaSectionMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "section-truss-menu-button",
                "section-beam-menu-button",
                "section-plate-menu-button",
            ],

            menuNames: {
                "section-truss-menu-button": "section-truss-menu",
                "section-beam-menu-button": "section-beam-menu",
                "section-plate-menu-button": "section-plate-menu",
            },

            captions: {
                "section-truss-menu-button": "Truss",
                "section-beam-menu-button": "Beam",
                "section-plate-menu-button": "Plate",
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
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                width: 12rem;
            }

            .section-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .section-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .section-truss-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.0rem;
                margin-right: 0rem;
            }

            .section-truss-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 2.3rem;
                height: 1.7rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .section-truss-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 2.3rem;
                height: 1.7rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .section-truss-menu-button:hover .section-truss-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .section-truss-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .section-truss-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .section-truss-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .section-truss-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }

            .section-beam-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .section-beam-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 2.3rem;
                height: 1.7rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .section-beam-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 2.3rem;
                height: 1.7rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .section-beam-menu-button:hover .section-beam-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .section-beam-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .section-beam-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .section-beam-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .section-beam-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }

            .section-plate-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .section-plate-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 2.3rem;
                height: 1.7rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .section-plate-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 2.3rem;
                height: 1.7rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .section-plate-menu-button:hover .section-plate-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .section-plate-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .section-plate-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .section-plate-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .section-plate-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>

        <div class=wrapper>
            <div class="section-menu-buttons-content">

                <p class="section-menu-buttons-caption">Section</p>

                <button class="section-truss-menu-button">
                    <div class="section-truss-menu-button-icon-content">
                        <svg class="section-truss-menu-button-icon" width="50" height="37" viewBox="0 0 50 37" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Truss section</title>
                            <g stroke="currentColor">
                                <path d="M16.0991 1L4.09842 25.9567H26.0306M16.0991 1L26.0306 25.9567M16.0991 
                                    1H37.2036M26.0306 25.9567L37.2036 1M26.0306 25.9567H46.7214L37.2036 1"
                                />
                                <path d="M45.9016 26.955L48.0312 31.4472H43.7721L45.9016 26.955Z"/>
                                <ellipse cx="44.2623" cy="32.9446" rx="0.819672" ry="0.99827" stroke-width="0.5"/>
                                <ellipse cx="4.50823" cy="32.4455" rx="1.22951" ry="1.4974" stroke-width="0.5"/>
                                <ellipse cx="47.5409" cy="32.9446" rx="0.819672" ry="0.99827" stroke-width="0.5"/>
                                <line x1="41.8033" y1="34.6912" x2="50.0001" y2="34.6912" stroke-width="0.5"/>
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 42.6727 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 44.3121 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584
                                    -0.772853 45.9514 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 47.5907 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 49.2301 34.9412)" stroke-width="0.5"
                                />
                                <line x1="0.819702" y1="34.6912" x2="8.19675" y2="34.6912" stroke-width="0.5"/>
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 0.869415 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 2.50873 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 4.14804 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 5.78745 34.9412)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="1.37002" y2="-0.25" transform="matrix(-0.634584 0.772853 -0.634584 
                                    -0.772853 7.42676 34.9412)" stroke-width="0.5"
                                />
                                <line x1="4.34842" y1="25.9567" x2="4.34842" y2="30.9481" stroke-width="0.5"/>
                                <path d="M26.0527 36.1162C26.1504 36.2138 26.3087 36.2138 26.4063 36.1162L27.9973 
                                    34.5252C28.0949 34.4276 28.0949 34.2693 27.9973 34.1717C27.8997 34.074 27.7414 
                                    34.074 27.6437 34.1717L26.2295 35.5859L24.8153 34.1717C24.7177 34.074 24.5594 
                                    34.074 24.4618 34.1717C24.3641 34.2693 24.3641 34.4276 24.4618 34.5252L26.0527 
                                    36.1162ZM25.9795 25.9567V35.9394H26.4795V25.9567H25.9795Z"
                                />
                            </g>
                        </svg>
                    </div>
                </button>

                <button class="section-beam-menu-button">
                    <div class="section-beam-menu-button-icon-content">
                        <svg class="section-beam-menu-button-icon" width="53" height="36" viewBox="0 0 53 36" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Beam section</title>
                            <g stroke="currentColor">
                                <path d="M2.09842 23.7126H46.7979M2.09842 7.10491H46.7979M2.09842 
                                    26.3349V4.52045H46.7979V26.3349H25.0991H2.09842Z"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 0.0606537)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 24.5352)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 22.787)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 21.0388)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 19.2906)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 17.5424)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 15.7943)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 14.0461)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 12.2979)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 10.5497)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 8.80155)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 7.05337)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 5.30519)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 3.55701)" stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" transform="matrix(-0.768165 0.640252 
                                    -0.625897 -0.779906 52.0984 1.80883)" stroke-width="0.5"
                                />
                                <path d="M2.05915 35.2374C2.15679 35.3351 2.31508 35.3351 2.41271 
                                    35.2374L4.0037 33.6464C4.10133 33.5488 4.10133 33.3905 4.0037 
                                    33.2929C3.90607 33.1953 3.74778 33.1953 3.65014 33.2929L2.23593 
                                    34.7071L0.821718 33.2929C0.724087 33.1953 0.565796 33.1953 0.468164 
                                    33.2929C0.370533 33.3905 0.370533 33.5488 0.468164 33.6464L2.05915 
                                    35.2374ZM1.98593 26.3349L1.98593 35.0607H2.48593L2.48593 26.3349H1.98593Z"
                                />
                                <line x1="47.5901" y1="1.89818" x2="47.5901" y2="28.995"/>
                            </g>
                        </svg>

                    </div>
                </button>

                <button class="section-plate-menu-button">
                    <div class="section-plate-menu-button-icon-content">
                        <svg class="section-plate-menu-button-icon" width="54" height="35" viewBox="0 0 54 35" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Plate section</title>
                            <g stroke="currentColor">
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 53.0897 22.9217)" 
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 52.3806 21.3238)"
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 51.6714 19.7259)" 
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 50.9623 18.128)" 
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 50.2532 16.5301)" 
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 49.5441 14.9322)"
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 48.8349 13.3343)" 
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 48.1258 11.7365)" 
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 47.4167 10.1385)"
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 46.7075 8.54066)"
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 45.9984 6.94276)"
                                    stroke-width="0.5"
                                />
                                <line y1="-0.25" x2="5.73425" y2="-0.25" 
                                    transform="matrix(-0.442418 0.896809 -0.888451 -0.458972 45.2893 5.34485)"
                                    stroke-width="0.5"
                                />
                                <path d="M12.3947 26.3557C12.4924 26.4534 12.6506 26.4534 12.7483 26.3557L14.3393 
                                    24.7648C14.4369 24.6671 14.4369 24.5088 14.3393 24.4112C14.2416 24.3136 14.0833 
                                    24.3136 13.9857 24.4112L12.5715 25.8254L11.1573 24.4112C11.0597 24.3136 10.9014 
                                    24.3136 10.8037 24.4112C10.7061 24.5088 10.7061 24.6671 10.8037 24.7648L12.3947 
                                    26.3557ZM12.3215 11.4436V26.179H12.8215V11.4436H12.3215Z"
                                />
                                <path d="M6.60091 14.9122C6.69854 15.0098 6.85683 15.0098 6.95446 14.9122L8.54545 
                                    13.3212C8.64309 13.2235 8.64309 13.0652 8.54545 12.9676C8.44782 12.87 8.28953 
                                    12.87 8.1919 12.9676L6.77769 14.3818L5.36347 12.9676C5.26584 12.87 5.10755 12.87 
                                    5.00992 12.9676C4.91229 13.0652 4.91229 13.2235 5.00992 13.3212L6.60091 
                                    14.9122ZM6.52769 0V14.7354H7.02769V0L6.52769 0Z"
                                />
                                <path d="M41.4807 26.3557C41.5784 26.4534 41.7366 26.4534 41.8343 26.3557L43.4253 
                                    24.7648C43.5229 24.6671 43.5229 24.5088 43.4253 24.4112C43.3276 24.3136 43.1693 
                                    24.3136 43.0717 24.4112L41.6575 25.8254L40.2433 24.4112C40.1457 24.3136 39.9874 
                                    24.3136 39.8897 24.4112C39.7921 24.5088 39.7921 24.6671 39.8897 24.7648L41.4807 
                                    26.3557ZM41.4075 11.4436V26.179H41.9075V11.4436H41.4075Z"
                                />
                                <path d="M35.6869 14.9122C35.7845 15.0098 35.9428 15.0098 36.0405 14.9122L37.6314 
                                    13.3212C37.7291 13.2235 37.7291 13.0652 37.6314 12.9676C37.5338 12.87 37.3755 
                                    12.87 37.2779 12.9676L35.8637 14.3818L34.4495 12.9676C34.3518 12.87 34.1935 12.87 
                                    34.0959 12.9676C33.9983 13.0652 33.9983 13.2235 34.0959 13.3212L35.6869 
                                    14.9122ZM35.6137 0V14.7354H36.1137V0L35.6137 0Z"
                                />
                                <path d="M31.3639 26.3557C31.4615 26.4534 31.6198 26.4534 31.7174 26.3557L33.3084 
                                    24.7648C33.406 24.6671 33.406 24.5088 33.3084 24.4112C33.2108 24.3136 33.0525 
                                    24.3136 32.9548 24.4112L31.5406 25.8254L30.1264 24.4112C30.0288 24.3136 29.8705 
                                    24.3136 29.7729 24.4112C29.6752 24.5088 29.6752 24.6671 29.7729 24.7648L31.3639 
                                    26.3557ZM31.2906 11.4436V26.179H31.7906V11.4436H31.2906Z"
                                />
                                <path d="M25.57 14.9122C25.6677 15.0098 25.826 15.0098 25.9236 14.9122L27.5146 
                                    13.3212C27.6122 13.2235 27.6122 13.0652 27.5146 12.9676C27.4169 12.87 27.2587 
                                    12.87 27.161 12.9676L25.7468 14.3818L24.3326 12.9676C24.235 12.87 24.0767 12.87 
                                    23.979 12.9676C23.8814 13.0652 23.8814 13.2235 23.979 13.3212L25.57 
                                    14.9122ZM25.4968 0V14.7354H25.9968V0L25.4968 0Z"
                                />
                                <path d="M21.247 26.3557C21.3446 26.4534 21.5029 26.4534 21.6006 26.3557L23.1915 
                                    24.7648C23.2892 24.6671 23.2892 24.5088 23.1915 24.4112C23.0939 24.3136 22.9356
                                    24.3136 22.838 24.4112L21.4238 25.8254L20.0096 24.4112C19.9119 24.3136 19.7536 
                                    24.3136 19.656 24.4112C19.5584 24.5088 19.5584 24.6671 19.656 24.7648L21.247 
                                    26.3557ZM21.1738 11.4436V26.179H21.6738V11.4436H21.1738Z"
                                />
                                <path d="M15.4532 14.9122C15.5508 15.0098 15.7091 15.0098 15.8067 14.9122L17.3977 
                                    13.3212C17.4953 13.2235 17.4953 13.0652 17.3977 12.9676C17.3001 12.87 17.1418 
                                    12.87 17.0442 12.9676L15.6299 14.3818L14.2157 12.9676C14.1181 12.87 13.9598 
                                    12.87 13.8622 12.9676C13.7645 13.0652 13.7645 13.2235 13.8622 13.3212L15.4532 
                                    14.9122ZM15.3799 0V14.7354H15.8799V0L15.3799 0Z"
                                />
                                <path d="M1 9.66922H42.0998L50.6359 29.0895H9.21995L1 9.66922Z" 
                                    stroke-linejoin="round"
                                />
                                <path d="M1 14.7354L9.21995 34.1556H50.6359" stroke-linejoin="round"/>
                                <line x1="0.5" y1="14.7354" x2="0.5" y2="9.66921"/>
                                <line x1="50.452" y1="34" x2="50.452" y2="28.9338"/>
                                <line x1="9.35225" y1="34" x2="9.35225" y2="28.9338"/>
                            </g>
                        </svg>
                    </div>
                </button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".section-truss-menu-button").addEventListener("click", 
            () => this.activate("section-truss-menu-button"));

        this.shadowRoot.querySelector(".section-beam-menu-button").addEventListener("click", 
            () => this.activate("section-beam-menu-button"));
        
        this.shadowRoot.querySelector(".section-plate-menu-button").addEventListener("click", 
            () => this.activate("section-plate-menu-button"));
    }

    connectedCallback() {
        this.activate("section-truss-menu-button");
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

    activate(buttonName) {
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
        if (currentButton.classList.contains("active") === false) {
            currentButton.classList.add("active");
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".section-menu-buttons-caption").innerHTML = caption;
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

export default FeaSectionMenuButtons;
