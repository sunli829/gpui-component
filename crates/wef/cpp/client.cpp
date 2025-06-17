#include "client.h"

#include <iostream>

#include "include/base/cef_bind.h"
#include "include/base/cef_callback.h"
#include "include/cef_browser.h"
#include "include/cef_task.h"
#include "include/wrapper/cef_closure_task.h"
#include "wef.h"

WefClient::WefClient(WefBrowser* wef_browser, float device_scale_factor,
                     int width, int height, BrowserCallbacks callbacks,
                     void* userdata, DestroyFn destroy_userdata)
    : wef_browser_(wef_browser),
      width_(width),
      height_(height),
      device_scale_factor_(device_scale_factor),
      callbacks_(callbacks),
      userdata_(userdata),
      destroy_userdata_(destroy_userdata) {}

WefClient::~WefClient() {
  if (wef_browser_) {
    if (wef_browser_->browser) {
      (*wef_browser_->browser)->GetHost()->CloseBrowser(true);
    }
    delete wef_browser_;
  }

  if (destroy_userdata_) {
    destroy_userdata_(userdata_);
  }
}

/////////////////////////////////////////////////////////////////
// CefRenderHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnPopupShow(CefRefPtr<CefBrowser> browser, bool show) {
  DCHECK(CefCurrentlyOn(TID_UI));
  callbacks_.on_popup_show(userdata_, show);
}

void WefClient::OnPopupSize(CefRefPtr<CefBrowser> browser,
                            const CefRect& rect) {
  DCHECK(CefCurrentlyOn(TID_UI));
  callbacks_.on_popup_position(userdata_, &rect);
}

void WefClient::OnPaint(CefRefPtr<CefBrowser> browser, PaintElementType type,
                        const RectList& dirtyRects, const void* buffer,
                        int width, int height) {
  DCHECK(CefCurrentlyOn(TID_UI));
  callbacks_.on_paint(userdata_, static_cast<int>(type), &dirtyRects, buffer,
                      static_cast<uint32_t>(width),
                      static_cast<uint32_t>(height));
}

void WefClient::OnImeCompositionRangeChanged(CefRefPtr<CefBrowser> browser,
                                             const CefRange& selected_range,
                                             const RectList& character_bounds) {
  DCHECK(CefCurrentlyOn(TID_UI));

  int xmin = std::numeric_limits<int>::max();
  int ymin = std::numeric_limits<int>::max();
  int xmax = std::numeric_limits<int>::min();
  int ymax = std::numeric_limits<int>::min();

  for (const auto& r : character_bounds) {
    if (r.x < xmin) {
      xmin = r.x;
    }
    if (r.y < ymin) {
      ymin = r.y;
    }
    if (r.x + r.width > xmax) {
      xmax = r.x + r.width;
    }
    if (r.y + r.height > ymax) {
      ymax = r.y + r.height;
    }
  }

  CefRect rect{int(float(xmin)), int(float(ymin)), int(float(xmax - xmin)),
               int(float(ymax - ymin))};
  callbacks_.on_ime_composition_range_changed(userdata_, &rect);
}

bool WefClient::OnCursorChange(CefRefPtr<CefBrowser> browser,
                               CefCursorHandle cursor, cef_cursor_type_t type,
                               const CefCursorInfo& custom_cursor_info) {
  DCHECK(CefCurrentlyOn(TID_UI));

  return callbacks_.on_cursor_changed(
      userdata_, static_cast<int>(type),
      type == CT_CUSTOM ? &custom_cursor_info : nullptr);
}

/////////////////////////////////////////////////////////////////
// CefDisplayHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnAddressChange(CefRefPtr<CefBrowser> browser,
                                CefRefPtr<CefFrame> frame,
                                const CefString& url) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto url_str = url.ToString();
  callbacks_.on_address_changed(userdata_, new WefFrame{frame},
                                url_str.c_str());
}

void WefClient::OnTitleChange(CefRefPtr<CefBrowser> browser,
                              const CefString& title) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto title_str = title.ToString();
  callbacks_.on_title_changed(userdata_, title_str.c_str());
}

void WefClient::OnFaviconURLChange(CefRefPtr<CefBrowser> browser,
                                   const std::vector<CefString>& icon_urls) {
  DCHECK(CefCurrentlyOn(TID_UI));

  std::vector<std::string> str_urls;
  std::transform(icon_urls.begin(), icon_urls.end(),
                 std::back_inserter(str_urls),
                 [](const CefString& url) { return url.ToString(); });

  std::vector<const char*> cstr_urls;
  std::transform(str_urls.begin(), str_urls.end(),
                 std::back_inserter(cstr_urls),
                 [](const std::string& url) { return url.c_str(); });

  callbacks_.on_favicon_url_change(userdata_, cstr_urls.data(),
                                   static_cast<int>(cstr_urls.size()));
}

bool WefClient::OnTooltip(CefRefPtr<CefBrowser> browser, CefString& text) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto text_str = text.ToString();
  callbacks_.on_tooltip(userdata_, text_str.c_str());
  return true;
}

void WefClient::OnStatusMessage(CefRefPtr<CefBrowser> browser,
                                const CefString& value) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto text_str = value.ToString();
  callbacks_.on_status_message(userdata_, text_str.c_str());
}

bool WefClient::OnConsoleMessage(CefRefPtr<CefBrowser> browser,
                                 cef_log_severity_t level,
                                 const CefString& message,
                                 const CefString& source, int line) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto message_str = message.ToString();
  auto source_str = source.ToString();
  callbacks_.on_console_message(userdata_, message_str.c_str(),
                                static_cast<int>(level), source_str.c_str(),
                                line);
  return false;
}

void WefClient::OnLoadingProgressChange(CefRefPtr<CefBrowser> browser,
                                        double progress) {
  DCHECK(CefCurrentlyOn(TID_UI));

  callbacks_.on_loading_progress_changed(userdata_,
                                         static_cast<float>(progress));
}

/////////////////////////////////////////////////////////////////
// CefLifeSpanHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnAfterCreated(CefRefPtr<CefBrowser> browser) {
  CefMessageRouterConfig config;
  message_router_ = CefMessageRouterBrowserSide::Create(config);
  message_router_->AddHandler(this, false);

  wef_browser_->browser = browser;
  if (!wef_browser_->url.empty()) {
    browser->GetMainFrame()->LoadURL(wef_browser_->url);
  }
  callbacks_.on_created(userdata_);

  if (wef_browser_->closeBrowser) {
    CefPostTask(TID_UI, base::BindOnce(&CefBrowserHost::CloseBrowser,
                                       browser->GetHost(), false));
    return;
  }
}

bool WefClient::OnBeforePopup(
    CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, int popup_id,
    const CefString& target_url, const CefString& target_frame_name,
    CefLifeSpanHandler::WindowOpenDisposition target_disposition,
    bool user_gesture, const CefPopupFeatures& popupFeatures,
    CefWindowInfo& windowInfo, CefRefPtr<CefClient>& client,
    CefBrowserSettings& settings, CefRefPtr<CefDictionaryValue>& extra_info,
    bool* no_javascript_access) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto target_url_str = target_url.ToString();
  callbacks_.on_before_popup(userdata_, target_url_str.c_str());
  return true;
}

bool WefClient::DoClose(CefRefPtr<CefBrowser> browser) { return false; }

void WefClient::OnBeforeClose(CefRefPtr<CefBrowser> browser) {
  DCHECK(CefCurrentlyOn(TID_UI));

  message_router_->OnBeforeClose(browser);
  delete wef_browser_;
  wef_browser_ = nullptr;

  callbacks_.on_closed(userdata_);
}

/////////////////////////////////////////////////////////////////
// CefLoadHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnLoadingStateChange(CefRefPtr<CefBrowser> browser,
                                     bool isLoading, bool canGoBack,
                                     bool canGoForward) {
  DCHECK(CefCurrentlyOn(TID_UI));

  callbacks_.on_loading_state_changed(userdata_, isLoading, canGoBack,
                                      canGoForward);
}

void WefClient::OnLoadStart(CefRefPtr<CefBrowser> browser,
                            CefRefPtr<CefFrame> frame,
                            TransitionType transition_type) {
  DCHECK(CefCurrentlyOn(TID_UI));

  callbacks_.on_load_start(userdata_, new WefFrame{frame});
}

void WefClient::OnLoadEnd(CefRefPtr<CefBrowser> browser,
                          CefRefPtr<CefFrame> frame, int httpStatusCode) {
  DCHECK(CefCurrentlyOn(TID_UI));

  callbacks_.on_load_end(userdata_, new WefFrame{frame});
  if (wef_browser_->browser) {
    (*wef_browser_->browser)->GetHost()->SetFocus(wef_browser_->focus);
  }
}

void WefClient::OnLoadError(CefRefPtr<CefBrowser> browser,
                            CefRefPtr<CefFrame> frame, ErrorCode errorCode,
                            const CefString& errorText,
                            const CefString& failedUrl) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto error_text_str = errorText.ToString();
  auto failed_url_str = failedUrl.ToString();
  callbacks_.on_load_error(userdata_, new WefFrame{frame},
                           error_text_str.c_str(), failed_url_str.c_str());
}

/////////////////////////////////////////////////////////////////
// CefDialogHandler methods
/////////////////////////////////////////////////////////////////
bool WefClient::OnFileDialog(CefRefPtr<CefBrowser> browser, FileDialogMode mode,
                             const CefString& title,
                             const CefString& default_file_path,
                             const std::vector<CefString>& accept_filters,
                             const std::vector<CefString>& accept_extensions,
                             const std::vector<CefString>& accept_descriptions,
                             CefRefPtr<CefFileDialogCallback> callback) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto title_str = title.ToString();
  auto default_file_path_str = default_file_path.ToString();
  auto accept_filters_str = join_strings(accept_filters, "@@@");
  auto accept_extensions_str = join_strings(accept_extensions, "@@@");
  auto accept_descriptions_str = join_strings(accept_descriptions, "@@@");
  CefRefPtr<CefFileDialogCallback>* callback_ptr =
      new CefRefPtr<CefFileDialogCallback>(callback);
  return callbacks_.on_file_dialog(
      userdata_, static_cast<int>(mode), title_str.c_str(),
      default_file_path_str.c_str(), accept_filters_str.c_str(),
      accept_extensions_str.c_str(), accept_descriptions_str.c_str(),
      callback_ptr);
}

/////////////////////////////////////////////////////////////////
// CefContextMenuHandler methods
/////////////////////////////////////////////////////////////////
bool WefClient::RunContextMenu(CefRefPtr<CefBrowser> browser,
                               CefRefPtr<CefFrame> frame,
                               CefRefPtr<CefContextMenuParams> params,
                               CefRefPtr<CefMenuModel> model,
                               CefRefPtr<CefRunContextMenuCallback> callback) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto link_url_str = params->GetLinkUrl().ToString();
  auto unfiltered_link_url_str = params->GetUnfilteredLinkUrl().ToString();
  auto source_url_str = params->GetSourceUrl().ToString();
  auto title_text_str = params->GetTitleText().ToString();
  auto page_url_str = params->GetPageUrl().ToString();
  auto frame_url_str = params->GetFrameUrl().ToString();
  auto selection_text_str = params->GetSelectionText().ToString();

  _ContextMenuParams params_{
      params->GetXCoord(),
      params->GetYCoord(),
      static_cast<int>(params->GetTypeFlags()),
      !link_url_str.empty() ? link_url_str.c_str() : nullptr,
      !unfiltered_link_url_str.empty() ? unfiltered_link_url_str.c_str()
                                       : nullptr,
      !source_url_str.empty() ? source_url_str.c_str() : nullptr,
      params->HasImageContents(),
      !title_text_str.empty() ? title_text_str.c_str() : nullptr,
      page_url_str.c_str(),
      frame_url_str.c_str(),
      static_cast<int>(params->GetMediaType()),
      static_cast<int>(params->GetMediaStateFlags()),
      selection_text_str.c_str(),
      params->IsEditable(),
      static_cast<int>(params->GetEditStateFlags()),
  };
  callbacks_.on_context_menu(userdata_, new WefFrame{frame}, &params_);
  return true;
}

/////////////////////////////////////////////////////////////////
// CefFindHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnFindResult(CefRefPtr<CefBrowser> browser, int identifier,
                             int count, const CefRect& selectionRect,
                             int activeMatchOrdinal, bool finalUpdate) {
  DCHECK(CefCurrentlyOn(TID_UI));

  callbacks_.on_find_result(userdata_, identifier, count, &selectionRect,
                            activeMatchOrdinal, finalUpdate);
}

/////////////////////////////////////////////////////////////////
// CefJSDialogHandler methods
/////////////////////////////////////////////////////////////////
bool WefClient::OnJSDialog(CefRefPtr<CefBrowser> browser,
                           const CefString& origin_url,
                           JSDialogType dialog_type,
                           const CefString& message_text,
                           const CefString& default_prompt_text,
                           CefRefPtr<CefJSDialogCallback> callback,
                           bool& suppress_message) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto message_text_str = message_text.ToString();
  auto default_prompt_text_str = default_prompt_text.ToString();
  CefRefPtr<CefJSDialogCallback>* callback_ptr =
      new CefRefPtr<CefJSDialogCallback>(callback);

  return callbacks_.on_js_dialog(userdata_, static_cast<int>(dialog_type),
                                 message_text_str.c_str(),
                                 default_prompt_text_str.c_str(), callback_ptr);
}

bool WefClient::OnBeforeUnloadDialog(CefRefPtr<CefBrowser> browser,
                                     const CefString& message_text,
                                     bool is_reload,
                                     CefRefPtr<CefJSDialogCallback> callback) {
  callback->Continue(true, "");
  return true;
}

/////////////////////////////////////////////////////////////////
// CefRequestHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnRenderProcessTerminated(CefRefPtr<CefBrowser> browser,
                                          TerminationStatus status,
                                          int error_code,
                                          const CefString& error_string) {
  message_router_->OnRenderProcessTerminated(browser);
}

bool WefClient::OnBeforeBrowse(CefRefPtr<CefBrowser> browser,
                               CefRefPtr<CefFrame> frame,
                               CefRefPtr<CefRequest> request, bool user_gesture,
                               bool is_redirect) {
  message_router_->OnBeforeBrowse(browser, frame);
  return false;
}

/////////////////////////////////////////////////////////////////
// CefFocusHandler methods
/////////////////////////////////////////////////////////////////
void WefClient::OnTakeFocus(CefRefPtr<CefBrowser> browser, bool next) {}

bool WefClient::OnSetFocus(CefRefPtr<CefBrowser> browser, FocusSource source) {
  return false;
}

/////////////////////////////////////////////////////////////////
// CefPermissionHandler methods
/////////////////////////////////////////////////////////////////
bool WefClient::OnRequestMediaAccessPermission(
    CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
    const CefString& requesting_origin, uint32_t requested_permissions,
    CefRefPtr<CefMediaAccessCallback> callback) {
  callback->Continue(CEF_MEDIA_PERMISSION_NONE);
  return true;
}

/////////////////////////////////////////////////////////////////
// CefMessageRouterBrowserSide::Handler methods
/////////////////////////////////////////////////////////////////
bool WefClient::OnQuery(
    CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, int64_t query_id,
    const CefString& request, bool persistent,
    CefRefPtr<CefMessageRouterBrowserSide::Handler::Callback> callback) {
  DCHECK(CefCurrentlyOn(TID_UI));

  auto request_str = request.ToString();
  CefRefPtr<CefMessageRouterBrowserSide::Handler::Callback>* callback_ptr =
      new CefRefPtr<CefMessageRouterBrowserSide::Handler::Callback>(callback);
  callbacks_.on_query(userdata_, new WefFrame{frame}, request_str.c_str(),
                      callback_ptr);
  return true;
}
