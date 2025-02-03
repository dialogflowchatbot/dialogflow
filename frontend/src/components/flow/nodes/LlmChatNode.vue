<script setup>
import { inject, reactive, ref, onMounted, nextTick } from 'vue';
import { copyProperties, httpReq, getDefaultBranch } from '../../../assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const nodeData = reactive({
    nodeName: 'Llm chat node',
    brief: '',
    prompt: '',
    promptText: '',
    nodeExitType: 'exitByIntent',
    contextLength: 5,
    exitCondition: { "": "" },
    exitIntent: '',
    exitSpecialInputs: '',
    maxChatTimes: 1,
    responseStreaming: false,
    connectTimeout: 1000,
    readTimeout: 10000,
    whenTimeoutThen: 'GotoAnotherNode',
    valid: false,
    invalidMessages: [],
    branches: [],
    newNode: true,
});
const settings = reactive({})
const getNode = inject('getNode');
const { robotId } = inject('robotId');
// const { ollamaModels } = inject('ollamaModels');
const nodeSetFormVisible = ref(false)
const intents = reactive([])
const modelId = ref('')
const modelName = ref('')
const nodeName = ref();
const overrideTimeoutEnabled = ref(false)
const whenTimeoutThen = ref('GotoAnotherNode')
const responseAlternateText = ref('')

getNode().on("change:data", ({ current }) => {
    // console.log('toggled');
    // const { name, text, nodeSetFormVisible } = current;
    nodeSetFormVisible.value = true;
});

onMounted(async () => {
    // console.log('llmChatNode')
    const node = getNode();
    const data = node.getData();
    console.log('data', data);
    copyProperties(data, nodeData);
    // if (data) {
    //     if (data.nodeName)
    //         nodeData.nodeName = data.nodeName;
    //     nodeData.goToType = data.goToType;
    //     selectGotoType(nodeData.goToType);
    //     nodeData.goToSubFlowId = data.goToSubFlowId;
    //     nodeData.goToSubFlowName = data.goToSubFlowName;
    //     nodeData.brief = data.brief;
    //     nodeData.externalLink = data.externalLink;
    //     if (data.newNode)
    //         nodeData.newNode = data.newNode;
    // }
    if (nodeData.newNode) {
        nodeData.nodeName += data.nodeCnt.toString();
        // node.removePorts();
        node.addPort({
            group: 'absolute',
            args: { x: nodeName.value.offsetWidth - 15, y: 104 },
            markup: [
                { tagName: "circle", selector: "bopdy" },
                { tagName: "rect", selector: "bg" }
            ],
            attrs: {
                text: {
                    text: 'Next',
                    fontSize: 12,
                },
                // https://codesandbox.io/s/port-label-viwnos?file=/src/App.tsx
                bg: {
                    ref: "text",
                    refWidth: "100%",
                    refHeight: "110%",
                    refX: "-100%",
                    refX2: -15,
                    refY: -5,
                    fill: "rgb(235,238,245)"
                }
            },
        });
    }
    nodeData.newNode = false;
    validate();
    httpReq("GET", 'management/settings', { robotId: robotId }, null, null).then((res) => {
        // const r = res.json();
        if (res.data) {
            copyProperties(res.data, settings);
            updateBrief();
            if (nodeData.connectTimeout > 0 && nodeData.readTimeout > 0)
                overrideTimeoutEnabled.value = true;
            if (!nodeData.connectTimeout)
                nodeData.connectTimeout = settings.chatProvider.connectTimeoutMillis;
            if (!nodeData.readTimeout)
                nodeData.readTimeout = settings.chatProvider.readTimeoutMillis;
        }
    })
    if (typeof (nodeData.whenTimeoutThen) == 'string')
        whenTimeoutThen.value = nodeData.whenTimeoutThen;
    else {
        whenTimeoutThen.value = 'ResponseAlternateText'
        responseAlternateText.value = nodeData.whenTimeoutThen.ResponseAlternateText;
    }
    const r = await httpReq('GET', 'intent', { robotId: robotId }, null, null);
    // console.log(r);
    if (r.status == 200) {
        intents.splice(0, intents.length, ...r.data);
    }
})

const updateBrief = () => {
    modelId.value = settings.chatProvider.provider.id;
    modelName.value = settings.chatProvider.provider.model;
    let h = 'Chat model: ' + settings.chatProvider.provider.model + ' of ' + settings.chatProvider.provider.id;
    h += '<br/>History context length: ' + nodeData.contextLength;
    h += '<br/>Exit this node by: ' + nodeData.nodeExitType;
    nodeData.brief = h;
}

const validate = () => {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (!d.prompt)
        m.push('Please fill out "prompt" field.');
    if (d.nodeExitType == 'exitByIntent' && !d.exitIntent)
        m.push('Please select an intent to use for exiting.');
    if (d.nodeExitType == 'exitBySpecialInputs' && !d.exitSpecialInputs)
        m.push('Please type some special inputs for exiting.');
    if (whenTimeoutThen.value == 'ResponseAlternateText' && !responseAlternateText.value)
        m.push('Please enter alternate text.');
    d.valid = m.length == 0;
}

const saveForm = () => {
    const node = getNode();
    const ports = node.getPorts();
    const branch = getDefaultBranch();
    branch.branchName = ports[0].attrs.text.text;
    branch.branchId = ports[0].id;
    branch.branchType = 'GotoAnotherNode';
    nodeData.branches.splice(0, nodeData.branches.length, branch);
    validate()
    delete nodeData.exitCondition;
    nodeData.exitCondition = {};
    const nodeExitType = nodeData.nodeExitType.replace(/exitBy/, '');
    if (nodeExitType == 'Intent')
        nodeData.exitCondition[nodeExitType] = nodeData.exitIntent;
    else if (nodeExitType == 'SpecialInputs')
        nodeData.exitCondition[nodeExitType] = nodeData.exitSpecialInputs;
    else if (nodeExitType == 'MaxChatTimes')
        nodeData.exitCondition[nodeExitType] = nodeData.maxChatTimes;
    nodeData.prompt = JSON.stringify([{ "role": "user", "content": nodeData.promptText },])
    if (!overrideTimeoutEnabled.value) {
        nodeData.connectTimeout = null;
        nodeData.readTimeout = null;
    }
    delete nodeData.whenTimeoutThen;
    if (whenTimeoutThen.value == 'ResponseAlternateText') {
        nodeData.whenTimeoutThen = { ResponseAlternateText: responseAlternateText }
    } else {
        nodeData.whenTimeoutThen = whenTimeoutThen;
    }
    console.log('nodeData', nodeData);
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    updateBrief();
    hideForm();
}

const hideForm = () => {
    nodeSetFormVisible.value = false;
}

</script>
<style scoped>
.nodeBox {
    border: 2px #0000000e solid;
    height: 100%;
    width: 100%;
    background-color: white;
    font-size: 12px;
}

.nodeTitle {
    background-color: #6a2c70;
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
}

.optionWidth {
    width: 110px;
}
</style>
<template>
    <div class="nodeBox">
        <div ref="nodeName" class="nodeTitle">
            {{ nodeData.nodeName }}
            <span v-show="nodeData.invalidMessages.length > 0">
                <el-tooltip class="box-item" effect="dark" :content="nodeData.invalidMessages.join('<br/>')"
                    placement="bottom" raw-content>
                    <el-icon color="red" size="16">
                        <EpWarning />
                    </el-icon>
                </el-tooltip>
            </span>
        </div>
        <div v-html="nodeData.brief"></div>
        <!-- <teleport to="body"> -->
        <el-drawer v-if="nodeSetFormVisible" v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl"
            size="70%" :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="135px" :model="nodeData" style="max-width: 800px">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth" prop="nodeName" :rules="[
                    { required: true, message: 'nodeName is required' },
                ]">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="Chat model" :label-width="formLabelWidth">
                    {{ modelId }} - {{ modelName }}
                    (<router-link :to="{ name: 'settings', params: { robotId: robotId } }">change</router-link>)
                </el-form-item>
                <el-form-item label="Prompt" :label-width="formLabelWidth" prop="promptText" :rules="[
                    { required: true, message: 'Prompt is required' },
                ]">
                    <el-input v-model="nodeData.promptText" :rows="6" type="textarea" placeholder="" />
                </el-form-item>
                <el-form-item label="Context length" :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.contextLength" :min="0" :max="50" :step="5" />
                    How many chat history records will be added in prompt.
                </el-form-item>
                <el-form-item label="Exit condition" :label-width="formLabelWidth">
                    <el-radio-group v-model="nodeData.nodeExitType">
                        <el-radio value="exitByIntent">Intent</el-radio>
                        <el-radio value="exitBySpecialInputs">Special inputs</el-radio>
                        <el-radio value="exitByMaxChatTimes">Max chat times</el-radio>
                    </el-radio-group>
                </el-form-item>
                <!-- <el-form-item label="Response type" :label-width="formLabelWidth">
                    <el-radio-group v-model="nodeData.responseStreaming">
                        <el-radio value="enabled">Enabled</el-radio>
                        <el-radio value="disabled">Disabled</el-radio>
                    </el-radio-group>
                </el-form-item> -->
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-select v-model="nodeData.exitIntent" v-show="nodeData.nodeExitType == 'exitByIntent'">
                        <el-option v-for="item in intents" :key="item.id" :label="item.name" :value="item.id" />
                    </el-select>
                    <el-input v-model="nodeData.exitSpecialInputs"
                        v-show="nodeData.nodeExitType == 'exitBySpecialInputs'" />
                    <el-input-number v-model="nodeData.maxChatTimes" :min="1" :max="200" :step="1"
                        v-show="nodeData.nodeExitType == 'exitByMaxChatTimes'" />
                </el-form-item>
                <el-form-item label="Timeout" :label-width="formLabelWidth">
                    <el-checkbox label="Override timeout of settings" v-model="overrideTimeoutEnabled" />
                    <el-divider direction="vertical" />
                    Current settings are: [Connect: {{ settings.chatProvider?.connectTimeoutMillis }} millis, Read: {{
                        settings.chatProvider?.readTimeoutMillis }} mills]
                </el-form-item>
                <el-form-item label="Connection" :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.connectTimeout" :min="100" :max="65500"
                        :disabled="!overrideTimeoutEnabled" />
                </el-form-item>
                <el-form-item label="Read" :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.readTimeout" :min="200" :max="65500"
                        :disabled="!overrideTimeoutEnabled" />
                </el-form-item>
                <el-form-item label="When timeout then" :label-width="formLabelWidth">
                    <el-radio-group v-model="whenTimeoutThen">
                        <el-radio value="GotoAnotherNode">Goto next node</el-radio>
                        <el-radio value="ResponseAlternateText">Response alternate text</el-radio>
                        <el-radio value="DoNothing">Do nothing</el-radio>
                        <el-tooltip effect="light" placement="right">
                            <template #content>
                                Stay at this node until meet one of exiting condition.
                            </template>
                            <el-button size="small" circle>?</el-button>
                        </el-tooltip>
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Alternate text" :label-width="formLabelWidth"
                    v-show="whenTimeoutThen == 'ResponseAlternateText'">
                    <el-input v-model="responseAlternateText" />
                </el-form-item>
            </el-form>
            <div>
                <el-button type="primary" @click="saveForm()">{{ t('lang.common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('lang.common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>