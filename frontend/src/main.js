import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
// import ElementPlus from 'element-plus'
// import ElementTiptapPlugin from 'element-tiptap-vue3-fixed';
// import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import i18n from './assets/lang/i18n'
// import VueScrollTo from 'vue-scrollto'
import App from './App.vue'
// import Home from './components/Home.vue'
// import Intro from './components/Intro.vue'
// import Doc from './components/docs/Index.vue'
// import Tutorial from './components/docs/Tutorial.vue'
import GlobalSettings from './components/management/GlobalSettings.vue'
import Settings from './components/management/Settings.vue'
import MainFlow from './components/flow/MainFlow.vue'
import SubFlow from './components/flow/SubFlow.vue'
import IntentList from './components/knowledge/IntentList.vue'
import IntentDetail from './components/knowledge/IntentDetail.vue'
import Variable from './components/variable/Variable.vue'
import Home from './components/Home.vue'
// import Guide from './components/Guide.vue'
import HttpApiList from './components/external/HttpApiList.vue'
import HttpApiDetail from './components/external/HttpApiDetail.vue'
import RobotDetail from './components/robot/RobotDetail.vue'
import RobotFrame from './components/robot/RobotFrame.vue'
import DocList from './components/knowledge/DocList.vue'
import QnAList from './components/knowledge/QnAList.vue'

// import 'element-plus/dist/index.css'
// import ElementTiptapPlugin from 'element-tiptap-vue3-fixed';
// import 'element-tiptap-vue3-fixed/lib/style.css';

/*
const routes = [
  { path: '/', component: Home },
  // { path: '/introduction', component: Intro },
  // { path: '/docs', component: Doc },
  { path: '/demo/:demo', component: SubFlow },
  { path: '/mainflows/:robotId', name: 'mainflows', component: MainFlow },
  { path: '/subflow/:robotId/:id/:name', name: 'subflow', component: SubFlow, props: true },
  { path: '/settings', component: GlobalSettings },
  { path: '/settings/:robotId', name: 'settings', component: Settings },
  { path: '/robot/:robotId', name: 'robotDetail', component: RobotDetail },
  { path: '/intents/:robotId', name: 'intents', component: IntentList },
  { path: '/intent/detail', component: IntentDetail },
  { path: '/variables/:robotId', name: 'variables', component: Variable },
  // { path: '/tutorial', component: Tutorial },
  { path: '/external/httpApis/:robotId', name: 'externalHttpApis', component: HttpApiList },
  { path: '/external/httpApi/:robotId/:id', name: 'externalHttpApiDetail', component: HttpApiDetail },
  { path: '/kb/doc/:robotId', name: 'kbDoc', component: DocList },
  { path: '/kb/qa/:robotId', name: 'kbQA', component: QnAList },
]
*/

const routes = [
  { path: '/', component: Home },
  // { path: '/introduction', component: Intro },
  // { path: '/docs', component: Doc },
  { path: '/demo/:demo', component: SubFlow },
  {
    path: '/robot/:robotId', component: RobotFrame, children: [
      { path: '', name: 'robotDetail', component: RobotDetail },
      { path: '/robot/:robotId/mainflows', name: 'mainflows', component: MainFlow },
      { path: '/robot/:robotId/kb/doc', name: 'kbDoc', component: DocList },
      { path: '/robot/:robotId/kb/qa', name: 'kbQA', component: QnAList },
      { path: '/robot/:robotId/settings', name: 'settings', component: Settings },
      { path: '/robot/:robotId/intents', name: 'intents', component: IntentList },
      { path: '/robot/:robotId/intent/detail', component: IntentDetail },
      { path: '/robot/:robotId/variables', name: 'variables', component: Variable },
      { path: '/robot/:robotId/external/httpApis', name: 'externalHttpApis', component: HttpApiList },
      { path: '/robot/:robotId/external/httpApi/:id', name: 'externalHttpApiDetail', component: HttpApiDetail },
    ]
  },
  { path: '/robot/:robotId/subflow/:id/:name', name: 'subflow', component: SubFlow, props: true },
  { path: '/settings', component: GlobalSettings },
  // { path: '/tutorial', component: Tutorial },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior(to, from, savedPosition) {
    return { top: 0 }
  }
})

import './assets/main.css'
// import 'bootstrap/dist/css/bootstrap.min.css'
import 'virtual:uno.css'

// createApp(App).mount('#app')
const app = createApp(App)

// for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
// app.component(key, component)
// }

// app.use(ElementPlus)
// app.use(ElementTiptapPlugin)
app.use(router)
app.use(i18n)
// app.use(VueScrollTo)
app.mount('#app')
