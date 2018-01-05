/*
   +----------------------------------------------------------------------+
   | HipHop for PHP                                                       |
   +----------------------------------------------------------------------+
   | Copyright (c) 2017-present Facebook, Inc. (http://www.facebook.com)  |
   +----------------------------------------------------------------------+
   | This source file is subject to version 3.01 of the PHP license,      |
   | that is bundled with this package in the file LICENSE, and is        |
   | available through the world-wide-web at the following url:           |
   | http://www.php.net/license/3_01.txt                                  |
   | If you did not receive a copy of the PHP license and are unable to   |
   | obtain it through the world-wide-web, please send a note to          |
   | license@php.net so we can mail you a copy immediately.               |
   +----------------------------------------------------------------------+
*/

#include "hphp/runtime/ext/vsdebug/debugger.h"
#include "hphp/runtime/ext/vsdebug/command.h"
#include "hphp/runtime/base/backtrace.h"
#include "hphp/runtime/base/tv-variant.h"

namespace HPHP {
namespace VSDEBUG {

StackTraceCommand::StackTraceCommand(
  Debugger* debugger,
  folly::dynamic message
) : VSCommand(debugger, message) {
}

StackTraceCommand::~StackTraceCommand() {
}

const StaticString s_file("file");
const StaticString s_line("line");
const StaticString s_function("function");

bool StackTraceCommand::executeImpl(
  DebuggerSession* session,
  folly::dynamic* responseMsg
) {
  const int requestId = m_debugger->getCurrentThreadId();
  const folly::dynamic& message = getMessage();
  const folly::dynamic& args = tryGetObject(message, "arguments", s_emptyArgs);

  (*responseMsg)["body"] = folly::dynamic::object;

  folly::dynamic& stackTrace = (*responseMsg)["body"];
  stackTrace["stackFrames"] = folly::dynamic::array();
  folly::dynamic& frames = stackTrace["stackFrames"];

  if (m_debugger->getRequestInfo()->m_pauseRecurseCount == 0) {
    stackTrace["stackFrames"] = frames;
    stackTrace["totalFrames"] = 0;
    (*responseMsg)["body"] = stackTrace;
    return false;
  }

  int startFrame = tryGetInt(args, "startFrame", 0);
  int levels = tryGetInt(args, "levels", INT_MAX);
  int levelsAdded = 0;

  // Respond with a stack trace!
  auto backtraceArgs = BacktraceArgs()
                        .withSelf()
                        .withPseudoMain()
                        .setParserFrame(nullptr);

  const Array backtrace = createBacktrace(backtraceArgs);
  int backtraceSize = backtrace.size();
  const ClientPreferences& prefs = m_debugger->getClientPreferences();

  for (int depth = 0; depth < backtraceSize - 1; depth++) {
    if (depth < startFrame) {
      continue;
    }

    if (levelsAdded >= levels) {
      break;
    }

    auto const parentFrame = backtrace.rvalAt(depth + 1).unboxed();
    auto const funcName =
      tvCastToString(parentFrame.val().parr->get(s_function).tv()).data();
    auto const frame = backtrace.rvalAt(depth).unboxed();
    const auto file = frame.val().parr->get(s_file);
    const auto line = frame.val().parr->get(s_line);

    frames.push_back(folly::dynamic::object);
    folly::dynamic& stackFrame = frames[frames.size() - 1];
    stackFrame["id"] = session->generateFrameId(requestId, depth);
    stackFrame["name"] = funcName;

    int64_t lineNumber = tvCastToInt64(line.tv());
    if (!prefs.linesStartAt1) {
      lineNumber--;
    }

    stackFrame["line"] = lineNumber;
    stackFrame["column"] = 0;
    stackFrame["source"] = folly::dynamic::object;

    folly::dynamic& source = stackFrame["source"];
    std::string fileName = tvCastToString(file.tv()).toCppString();

    if (fileName.empty()) {
      // Some routines like builtins and native extensions do not have
      // a PHP file path in their frame's file name field.
      fileName = std::string("<unknown>");
    }

    source["name"] = fileName;
    source["path"] = fileName;

    levelsAdded++;
  }

  if (backtrace.size() == 0) {
    // The backtrace will be empty if the request is just starting up and hasn't
    // invoked anything yet.
    frames.push_back(folly::dynamic::object);
    folly::dynamic& stackFrame = frames[frames.size() - 1];
    stackFrame["id"] = -1;
    stackFrame["name"] = "{request initializing}";
    stackFrame["line"] = 0;
    stackFrame["column"] = 0;

    folly::dynamic source = folly::dynamic::object;
    source["name"] = "<unknown>";
    source["path"] = "<unknown>";
    stackFrame["source"] = source;
  }

  stackTrace["totalFrames"] = backtraceSize == 0 ? 0 : backtraceSize - 1;

  // Completion of this command does not resume the target.
  return false;
}

}
}
