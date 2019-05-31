import Basic from './basic';
import config from './config';
import { Selector, t } from 'testcafe';
  fixture `test Delete And Recovery`
    .page `${config.baseUrl}`;
  const basic = new Basic();
  const timeStamp = ''+Math.round(+new Date()/1000);
  const searchStartForm = "'rdfs:label' == '"+ timeStamp +"'" ;
  const query = "'rdfs:label' == '"+ timeStamp +"' && 'v-s:deleted' == 'true'" ;
  test('testDeleteAndRecovery', async t => {
    basic.login('karpovrt', '123');
    await t
      .click('#menu')
      .click('li[id="menu"] li[resource="v-s:Create"]')
      .typeText('veda-control.fulltext.dropdown', 'Стартовая форма')
      .click('div.suggestion[resource="v-wf:StartForm"]')
      .typeText('veda-control[data-type="multilingualString"] input[type="text"]', timeStamp)
      .click('#save')
      .wait(2000)
      .click('li[about="v-fs:FulltextSearch"]')
      .wait(5000)
      .typeText('veda-control[property="*"] input.form-control', searchStartForm)
      .click('div.input-group span.input-group-btn #custom-search-button.search-button')
      .click('div.search-result.noSwipe tbody.result-container a.glyphicon.glyphicon-search')
      .wait(5000)
      .setNativeDialogHandler(() => true)
      .click('#delete')
      .click('li[about="v-fs:FulltextSearch"]')
      .click('veda-control[property="*"] input.form-control')
      .pressKey('ctrl+a delete')
      .typeText('veda-control[property="*"] input.form-control', query)
      .click('div.input-group span.input-group-btn #custom-search-button.search-button')
      .click('div.results a.glyphicon.glyphicon-search.deleted[typeof="v-wf:StartForm"]')
      .click('p#deleted-alert-msg button#deleted-alert-recover')
      .click('li[about="v-fs:FulltextSearch"]')
      .click('.advanced-toggle')
      .click('div[rel="rdf:type"] .rel-actions button.btn.btn-default.button-delete')
      .click('veda-control[property="*"] input.form-control')
      .pressKey('ctrl+a delete')
      .typeText('veda-control[property="*"] input.form-control', timeStamp)
      .click('div.input-group span.input-group-btn #custom-search-button.search-button')
      .expect(Selector('small.stats-top.pull-right span[property="v-fs:estimated"]').innerText).eql('1');
  });
