<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router';
import { btoa, copyProperties, httpReq } from '../assets/tools.js'
import EpArrowRightBold from '~icons/ep/arrow-right-bold'
import EpSetting from '~icons/ep/setting'
import SolarDocumentTextLinear from '~icons/solar/document-text-linear'
import BiBoxArrowUpRight from '~icons/bi/box-arrow-up-right'
import RiRobot2Line from '~icons/ri/robot-2-line'
import OutboundCallBotAvatar from '@/assets/outbound-bot.png'
import InboundCallBotAvatar from '@/assets/inbound-bot.png'
import TextBotAvatar from '@/assets/text-bot.png'
useI18n();
const router = useRouter();
const currentVersion = ref('')
const checkUpdateResult = ref(0)
const updateLoading = ref(false)
const newVersion = ref('')
const changelog = reactive([])
const setFormVisible = ref(false)
const formLabelWidth = "90px"
const checkUpdate = async () => {
  updateLoading.value = true
  const t = await httpReq('GET', 'check-new-version.json', null, null, null);
  // console.log(t)
  if (t.status == 200) {
    if (t.data != null) {
      newVersion.value = t.data.version;
      changelog.splice(0, changelog.length)
      copyProperties(t.data.changelog, changelog)
      checkUpdateResult.value = 1
    } else {
      checkUpdateResult.value = 2
    }
  } else {
    checkUpdateResult.value = 3
  }
  updateLoading.value = false
}
const toSettings = () => {
  router.push('/settings')
}

const robots = reactive([])
const robotData = reactive({
  robotId: '',
  robotName: '',
  robotType: '',
})
onMounted(async () => {
  await list();
  const t = await httpReq('GET', 'version.json', null, null, null);
  currentVersion.value = t
});

async function list() {
  const t = await httpReq('GET', 'robot', null, null, null);
  if (t.status == 200) {
    robots.splice(0, robots.length, ...t.data.reverse());
  }
}

async function newRobot() {
  const t = await httpReq('POST', 'robot', null, null, robotData);
  // console.log(t.data);
  if (t.status == 200)
    await list();
  setFormVisible.value = false
}
function showRobotForm() {
  robotData.robotId = '';
  robotData.robotName = '';
  robotData.robotType = '';
  setFormVisible.value = true
}
function robotDetail(id, name) {
  router.push({ name: 'robotDetail', params: { robotId: id, name: btoa(name) } })
}
const getBotAvatar = (type) => {
  if (type == 'OutboundCallBot')
    return OutboundCallBotAvatar
  else if (type == 'InboundCallBot')
    return InboundCallBotAvatar
  else if (type == 'TextBot')
    return TextBotAvatar
  else
    return ''
}
const getBotType = (type) => {
  if (type == 'OutboundCallBot')
    return 'Telephone outbound bot'
  else if (type == 'InboundCallBot')
    return 'Telephone incoming bot'
  else if (type == 'TextBot')
    return 'Text chat bot'
  else
    return ''
}
const compareDifferentRobotTypeData = [
  {
    rtype: 'Telephone outbound bot',
    dialogNodeAnswerTextType: 'Plain text',
    llmChatNodeAsyncResponse: 'Not supported',
  },
  {
    rtype: 'Telephone incoming bot',
    dialogNodeAnswerTextType: 'Plain text',
    llmChatNodeAsyncResponse: 'Not supported',
  },
  {
    rtype: 'Text chat bot',
    dialogNodeAnswerTextType: 'Rich text',
    llmChatNodeAsyncResponse: 'Supported by SSE',
  },
];
</script>
<style scoped>
.header-row {
  margin-top: 20px;
}

.header {
  margin-left: 20px;
  font-size: 38px;
  font-weight: bold;
}

.title {
  font-size: 28px;
  font-weight: bold;
  margin-top: 35px;
}

.description {
  font-size: 16px;
  color: #b8b8b8;
  padding-bottom: 20px;
  border-bottom: #b8b8b8 1px solid;
}

.tips {
  text-align: right;
  margin-right: 30px;
}

#robotList .el-col {
  border-radius: 4px;
  margin-bottom: 20px;
}

.bg-color-light {
  border: #e5e9f2 1px solid;
  border-left: 3px solid #d3dce6;
  text-align: center;
  line-height: 30px;
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
}
</style>
<template>
  <!-- <el-page-header title="Home">
    <template #content>
      <span class="text-large font-600 mr-3"> Workspace </span>
    </template>
</el-page-header> -->
  <el-row class="header-row">
    <el-col :span="8">
      <span class="header"> Workspace </span>
      <!-- <el-button size="large">
        <el-icon size="large">
          <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"><path d="M21 4H7V2H5v20h2v-8h14l-2-5l2-5zm-3.86 5.74l.9 2.26H7V6h11.05l-.9 2.26l-.3.74l.29.74zM14 9c0 1.1-.9 2-2 2s-2-.9-2-2s.9-2 2-2s2 .9 2 2z" fill="currentColor"></path></svg>
        </el-icon>
      </el-button> -->
      <el-button size="large" :loading="updateLoading" @click="checkUpdate">
        <el-icon size="large">
          <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512">
            <path
              d="M256 504c137 0 248-111 248-248S393 8 256 8S8 119 8 256s111 248 248 248zm0-448c110.5 0 200 89.5 200 200s-89.5 200-200 200S56 366.5 56 256S145.5 56 256 56zm20 328h-40c-6.6 0-12-5.4-12-12V256h-67c-10.7 0-16-12.9-8.5-20.5l99-99c4.7-4.7 12.3-4.7 17 0l99 99c7.6 7.6 2.2 20.5-8.5 20.5h-67v116c0 6.6-5.4 12-12 12z"
              fill="currentColor"></path>
          </svg>
        </el-icon>
      </el-button>
      <el-button size="large" @click="toSettings">
        <el-icon size="large">
          <EpSetting />
        </el-icon>
      </el-button>
    </el-col>
  </el-row>
  <el-popover ref="popover" placement="right" title="Changelog" :width="300" trigger="hover">
    <template #reference>
      <el-button v-show="checkUpdateResult == 1" class="m-2" type="warning" text>Found new verion: {{
        newVersion
      }}</el-button>
    </template>
    <template #default>
      <ol style="margin:0;padding:0">
        <li v-for="(item, index) in changelog" :id="index" :key="index">
          {{ item }}
        </li>
      </ol>
      <a href="https://github.com/dialogflowchatbot/dialogflow/releases">Go to download</a>
    </template>
  </el-popover>
  <el-alert v-show="checkUpdateResult == 2" title="You're using the latest verion." type="success"
    @close="checkUpdateResult = 0" />
  <el-alert v-show="checkUpdateResult == 3" title="Failed to check update information, please try again later."
    type="danger" @close="checkUpdateResult = 0" />
  <!-- <el-button v-show="checkUpdateResult == 2" type="success" text>You're using the latest verion</el-button>
  <el-button v-show="checkUpdateResult == 3" type="danger" text>Failed to query update information, please try
    again
    later.</el-button> -->
  <div style="margin-left:50px">
    <el-row>
      <el-col :span="12">
        <h1>
          <el-icon :size="50">
            <RiRobot2Line />
          </el-icon>
          Choose a robot to start
          <el-button size="large" @click="showRobotForm" type="success">
            Create a new robot
          </el-button>
        </h1>
      </el-col>
      <!-- <el-col :span="12">
        <h1>
        </h1>
      </el-col> -->
    </el-row>
    <el-space wrap size="large">
      <div class="grid-content bg-color-light" v-for="n in robots" :key="n.robotId">
        <el-result :title="n.robotName" :sub-title="getBotType(n.robotType)">
          <template #icon>
            <el-image :src="getBotAvatar(n.robotType)" />
          </template>
          <template #extra>
            <el-button size="large" type="primary" @click="robotDetail(n.robotId, n.robotName)">Detail</el-button>
          </template>
        </el-result>
      </div>
    </el-space>
    <!-- <el-row id="robotList" :gutter="20">
      <el-col :xs="12" :sm="9" :md="7" :lg="5" v-for="i in robots" :key="i.robotId">
        <div class="grid-content bg-color-light">
          <el-result :title="i.robotName" :sub-title="getBotType(i.robotType)">
            <template #icon>
              <el-image :src="getBotAvatar(i.robotType)" />
            </template>
            <template #extra>
              <el-button type="primary" @click="robotDetail(i.robotId, i.robotName)">Detail</el-button>
            </template>
          </el-result>
        </div>
      </el-col>
    </el-row> -->
    <div class="title">
      <el-icon :size="30">
        <EpSetting />
      </el-icon>
      Global settings
    </div>
    <div>
      <el-icon :size="15">
        <EpArrowRightBold />
      </el-icon>
      <router-link to="/settings">Global settings</router-link>
      <div class="description">{{ $t('lang.guide.desc4') }}</div>
    </div>

    <div class="title">
      <el-icon :size="30">
        <SolarDocumentTextLinear />
      </el-icon>
      {{ $t('lang.guide.title5') }}
    </div>
    <div>
      <el-icon :size="15">
        <EpArrowRightBold />
      </el-icon>
      <!-- <router-link to="/docs">{{ $t('lang.guide.nav5') }}</router-link> -->
      <a href="https://dialogflowchatbot.github.io/doc" target="_blank">
        {{ $t('lang.guide.nav5') }}
        <el-icon>
          <BiBoxArrowUpRight />
        </el-icon>
      </a>
      <div class="description">{{ $t('lang.guide.desc5') }}</div>
    </div>
  </div>
  <div>
    <div class="text-center">
      Version: {{ currentVersion }}<br />
      <a href="https://dialogflowchatbot.github.io/" target="_blank">https://dialogflowchatbot.github.io/</a><br />
      If you have any questions or suggestions, please
      create a <a href="https://github.com/dialogflowchatbot/dialogflow/discussions" target="_blank">discussion</a> on
      Github
      or
      email to: dialogflow@yeah.net
    </div>
    <div class="text-center">
      Some icons were created by
      <a href="https://www.flaticon.com/" target="_blank">Flaticon</a>
    </div>
  </div>
  <el-dialog v-model="setFormVisible" title="Create a new robot" width="60%">
    <el-form :model="robotData">
      <el-form-item label="Name" :label-width="formLabelWidth" prop="robotName" :rules="[
        { required: true, message: 'Robot name is required' },
      ]">
        <el-input v-model="robotData.robotName" autocomplete="off" />
      </el-form-item>
      <el-form-item label="Type" :label-width="formLabelWidth" prop="robotType" :rules="[
        { required: true, message: 'Please choose a type of robot' },
      ]">
        <el-select v-model="robotData.robotType" placeholder="">
          <el-option label="Text bot" value="TextBot" />
          <el-option label="Inbound call bot" value="InboundCallBot" />
          <el-option label="Outbound call bot" value="OutboundCallBot" />
        </el-select>
      </el-form-item>
    </el-form>
    <el-table :data="compareDifferentRobotTypeData">
      <el-table-column property="rtype" label="" width="190" />
      <el-table-column property="dialogNodeAnswerTextType" label="Dialog node" width="150" />
      <el-table-column property="llmChatNodeAsyncResponse" label="Llm chat node streaming" width="200" />
    </el-table>
    <template #footer>
      <el-button type="primary" @click="newRobot()">Create</el-button>
      <el-button @click="setFormVisible = false">{{ $t('lang.common.cancel') }}</el-button>
    </template>
  </el-dialog>
</template>