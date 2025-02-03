<script setup>
import { inject, onMounted, reactive, ref } from 'vue'
import { copyProperties, getDefaultBranch, httpReq } from '../../../assets/tools.js'
import EpWarning from '~icons/ep/warning'
import { useI18n } from 'vue-i18n'
const { t, tm } = useI18n();

const nodeData = reactive({
    nodeName: 'Send email node',
    from: '',
    to: '',
    toRecipients: [],
    cc: '',
    ccRecipients: [],
    bcc: '',
    bccRecipients: [],
    subject: '',
    content: '',
    contentType: 'TextHtml',
    asyncSend: true,
    valid: false,
    invalidMessages: [],
    branches: [],
    newNode: true,
})
let lastTimeAsyncSendChoice = true;
const nodeName = ref();
const smtpHost = ref('')
const emailVerificationRegex = ref('')
const nodeSetFormVisible = ref(false)
const getNode = inject('getNode');
const { robotId } = inject('robotId');
const node = getNode();
const variables = []
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
onMounted(async () => {
    // console.log('emailNode')
    const data = node.getData();
    // console.log(data);
    copyProperties(data, nodeData);
    // if (data) {
    //     if (data.nodeName)
    //         nodeData.nodeName = data.nodeName;
    //     nodeData.collectType = data.collectType;
    //     nodeData.collectSaveVarName = data.collectSaveVarName;
    //     if (data.newNode)
    //         nodeData.newNode = data.newNode;
    // }
    // console.log(nodeData.newNode);
    if (nodeData.newNode) {
        nodeData.nodeName += data.nodeCnt.toString();
        resetPorts()
        nodeData.newNode = false;
        // console.log(nodeData);
    } else {
        lastTimeAsyncSendChoice = nodeData.asyncSend
    }
    let t = await httpReq('GET', 'variable', null, null, null);
    // console.log(t);
    if (t && t.status == 200 && t.data) {
        variables.splice(0, variables.length);
        t.data.forEach(function (item, index, arr) {
            this.push({ label: item.varName, value: item.varName });
        }, variables);
    }
    t = await httpReq('GET', 'management/settings', { robotId: robotId }, null, null);
    // console.log(t);
    if (t && t.status == 200 && t.data) {
        smtpHost.value = t.data.smtpHost
        // console.log(t.data.emailVerificationRegex)
        emailVerificationRegex.value = t.data.emailVerificationRegex
    }
    validate();
});
const resetPorts = () => {
    node.removePorts();
    const heightOffset = nodeName.value.offsetHeight + 50;
    const x = nodeName.value.offsetWidth - 15;
    if (nodeData.asyncSend) {
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset },
            attrs: {
                text: {
                    text: tm('lang.dialogNode.nextSteps')[1],
                    fontSize: 12,
                },
            },
        });
    } else {
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset },
            attrs: {
                text: {
                    text: tm('lang.collectNode.branches')[0],
                    fontSize: 12,
                },
            },
        });
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset + 20 },
            attrs: {
                text: {
                    text: tm('lang.collectNode.branches')[1],
                    fontSize: 12,
                },
            },
        });
        node.resize(node.size().width, 40 + heightOffset, { direction: 'bottom' })
    }
}
const validate = () => {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (!smtpHost.value)
        m.push('SMTP host is not configured');
    if (!d.nodeName)
        m.push('Need to fill in the node name');
    // console.log(emailVerificationRegex)
    const re = new RegExp(emailVerificationRegex.value);
    if (!d.to)
        m.push('Need to fill in the email recipient');
    else {
        d.to.split(';').forEach(function (item) {
            if (!item.match(re)) {
                m.push(item + ' is not a valid email format');
            } else d.toRecipients.push(item)
        })
    }
    if (d.cc) {
        d.cc.split(';').forEach(function (item) {
            if (!item.match(re)) {
                m.push(item + ' is not a valid email format');
            } else d.ccRecipients.push(item)
        })
    }
    if (d.bcc) {
        d.bcc.split(';').forEach(function (item) {
            if (!item.match(re)) {
                m.push(item + ' is not a valid email format');
            } else d.bccRecipients.push(item)
        })
    }
    if (!d.subject)
        m.push('Need to fill in the email subject');
    if (!d.content)
        m.push('Need to fill in the email content');
    if (d.branches == null || d.branches.length == 0)
        m.push('Wrong node branch information');
    // else {
    //     d.branches.forEach(function (item) {
    //         if (!item.target_node_id)
    //             m.push(item.branchName + ' is not connected to other nodes');
    //     })
    // }
    d.valid = m.length == 0;
}
const saveForm = () => {
    console.log(lastTimeAsyncSendChoice + "|" + nodeData.asyncSend)
    if (lastTimeAsyncSendChoice != nodeData.asyncSend) {
        lastTimeAsyncSendChoice = nodeData.asyncSend
        resetPorts()
    }
    const node = getNode();
    const ports = node.getPorts();
    nodeData.branches.splice(0, nodeData.branches.length);
    for (let i = 0; i < ports.length; i++) {
        const branch = getDefaultBranch();
        branch.branchName = ports[i].attrs.text.text;
        branch.branchId = ports[i].id;
        branch.branchType = i == 0 && ports.length > 1 ? 'EmailSentSuccessfully' : 'GotoAnotherNode';
        nodeData.branches.push(branch);
    }
    validate()
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
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
    background-color: rgb(255, 101, 85);
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
}
</style>
<template>
    <div class="nodeBox">
        <div ref="nodeName" class="nodeTitle">
            {{ nodeData.nodeName }}
            <span v-show="nodeData.invalidMessages.length > 0">
                <el-tooltip class="box-item" effect="dark" :content="nodeData.invalidMessages.join('<br/>')"
                    placement="bottom" raw-content>
                    <el-icon color="yellow" size="16">
                        <EpWarning />
                    </el-icon>
                </el-tooltip>
            </span>
        </div>
        <div>To: {{ nodeData.to }}</div>
        <div>Subject: {{ nodeData.subject }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="100px" :model="nodeData" style="max-width: 500px">
                <el-form-item :label="t('lang.common.nodeName')">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="From" :label-width="formLabelWidth" prop="from" :rules="[
                    {
                        required: true,
                        message: 'Please input email address',
                        trigger: 'blur',
                    },
                    {
                        type: 'email',
                        message: 'Please input correct email address',
                        trigger: ['blur', 'change'],
                    },
                ]">
                    <el-input v-model="nodeData.from" placeholder="" />
                </el-form-item>
                <el-form-item label="To" prop="to" :rules="[
                    {
                        required: true,
                        message: 'Please input email address',
                        trigger: 'blur',
                    },
                    {
                        type: 'email',
                        message: 'Please input correct email address',
                        trigger: ['blur', 'change'],
                    },
                ]">
                    <el-input v-model="nodeData.to" placeholder="" />
                </el-form-item>
                <el-form-item label="">
                    Separate multiple recipients with semicolons
                </el-form-item>
                <el-form-item label="Cc">
                    <el-input v-model="nodeData.cc" placeholder="" />
                </el-form-item>
                <el-form-item label="Bcc">
                    <el-input v-model="nodeData.bcc" placeholder="" />
                </el-form-item>
                <el-form-item label="Subject" prop="subject" :rules="[
                    { required: true, message: 'Subject is required' },
                ]">
                    <el-input v-model="nodeData.subject" placeholder="" />
                </el-form-item>
                <el-form-item label="Content" prop="content" :rules="[
                    { required: true, message: 'Content is required' },
                ]">
                    <el-input v-model="nodeData.content" :rows="2" type="textarea" placeholder="Please input" />
                </el-form-item>
                <el-form-item label="Content type">
                    <el-radio-group v-model="nodeData.contentType" class="ml-4">
                        <el-radio value="TextHtml" size="large">text/html</el-radio>
                        <el-radio value="TextPlain" size="large">text/plain</el-radio>
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="">
                    <input type="checkbox" id="_asyncSend_" v-model="nodeData.asyncSend"
                        :checked="nodeData.asyncSend" /><label for="_asyncSend_">Send asynchronously</label>
                </el-form-item>
            </el-form>
            <div class="demo-drawer__footer">
                <el-button type="primary" :loading="loading" @click="saveForm()">{{ t('lang.common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('lang.common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>
