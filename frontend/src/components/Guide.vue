<script setup>
import { ref, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router';
import { copyProperties, httpReq } from '../assets/tools.js'
import Demos from "./Demos.vue"
import EpArrowRightBold from '~icons/ep/arrow-right-bold'
import BiChatSquareDots from '~icons/bi/chat-square-dots'
import RiBardLine from '~icons/ri/bard-line'
import SolarDownloadOutline from '~icons/solar/download-outline'
import SolarRouting2Linear from '~icons/solar/routing-2-linear'
import EpSetting from '~icons/ep/setting'
import SolarDocumentTextLinear from '~icons/solar/document-text-linear'
import BiBoxArrowUpRight from '~icons/bi/box-arrow-up-right'
useI18n();
const router = useRouter();
const checkUpdateResult = ref(0)
const fromPage = 'guide';
const updateLoading = ref(false)
const newVersion = ref('')
const changelog = reactive([])
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
</style>
<template>
  <!-- <el-page-header title="Home">
    <template #content>
      <span class="text-large font-600 mr-3"> Workspace </span>
    </template>
</el-page-header> -->
  <div></div>
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
  <el-alert v-show="checkUpdateResult == 3" title="Failed to query update information, please try again later."
    type="danger" @close="checkUpdateResult = 0" />
  <!-- <el-button v-show="checkUpdateResult == 2" type="success" text>You're using the latest verion</el-button>
  <el-button v-show="checkUpdateResult == 3" type="danger" text>Failed to query update information, please try
    again
    later.</el-button> -->
  <p style="margin-left:50px">
  <div class="title">
    <el-icon :size="30">
      <BiChatSquareDots />
    </el-icon>{{ $t('lang.guide.title1') }}
  </div>
  <p>
    <el-icon :size="15">
      <EpArrowRightBold />
    </el-icon>
    <router-link to="/mainflows">{{ $t('lang.guide.nav1') }}</router-link>
  <div class="description">
    <Demos :parentPage="fromPage" />
    <!-- <router-link :to="{ name: 'subflow', params: { id: 'demo-repay', name: btoa('Repay Demo') } }">
            {{ $t('lang.home.demo1') }}
          </router-link>
          |
          <router-link
            :to="{ name: 'subflow', params: { id: 'demo-collect', name: btoa('Information Collection Demo') } }">
            {{ $t('lang.home.demo2') }}
          </router-link>
          |
          <router-link
            :to="{ name: 'subflow', params: { id: 'demo-notify', name: btoa('One Sentence Notification Demo') } }">
            {{ $t('lang.home.demo3') }}
          </router-link> -->
  </div>
  </p>

  <div class="title">
    <el-icon :size="30">
      <RiBardLine />
    </el-icon>
    {{ $t('lang.guide.title2') }}
  </div>
  <p>
    <el-icon :size="15">
      <EpArrowRightBold />
    </el-icon>
    <router-link to="/intents">{{ $t('lang.guide.nav2') }}</router-link>
  <div class="description">
    {{ $t('lang.guide.desc2') }}<br />
    We have built-in "Positive" and "Negative" intentions. If that's not enough, you can add your own
  </div>
  </p>

  <div class="title">
    <el-icon :size="30">
      <SolarDownloadOutline />
    </el-icon>
    {{ $t('lang.guide.title3') }}
  </div>
  <p>
    <el-icon :size="15">
      <EpArrowRightBold />
    </el-icon>
    <router-link to="/variables">{{ $t('lang.guide.nav3') }}</router-link>
  <div class="description">{{ $t('lang.guide.desc3') }}</div>
  </p>

  <div class="title">
    <el-icon :size="30">
      <SolarRouting2Linear />
    </el-icon>
    External APIs call
  </div>
  <p>
    <el-icon :size="15">
      <EpArrowRightBold />
    </el-icon>
    <router-link to="/external/httpApis">External HTTP API list</router-link>
  <div class="description">By using this function, you can send data to external URLs and receive response.</div>
  </p>

  <div class="title">
    <el-icon :size="30">
      <EpSetting />
    </el-icon>
    {{ $t('lang.guide.title4') }}
  </div>
  <p>
    <el-icon :size="15">
      <EpArrowRightBold />
    </el-icon>
    <router-link to="/settings">{{ $t('lang.guide.nav4') }}</router-link>
  <div class="description">{{ $t('lang.guide.desc4') }}</div>
  </p>

  <div class="title">
    <el-icon :size="30">
      <SolarDocumentTextLinear />
    </el-icon>
    {{ $t('lang.guide.title5') }}
  </div>
  <p>
    <el-icon :size="15">
      <EpArrowRightBold />
    </el-icon>
    <!-- <router-link to="/docs">{{ $t('lang.guide.nav5') }}</router-link> -->
    <a href="https://dialogflowchatbot.github.io/docs" target="_blank">
      {{ $t('lang.guide.nav5') }}
      <el-icon>
        <BiBoxArrowUpRight />
      </el-icon>
    </a>
  <div class="description">{{ $t('lang.guide.desc5') }}</div>
  </p>
  </p>
</template>